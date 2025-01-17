use std::{
    cell::{RefCell, Ref, RefMut},
    collections::BTreeMap,
    iter,
    path::Path,
    rc::Rc,
};

use cosmwasm_std::{Order, Record, Storage};
use cw_sdk::hash::HASH_LENGTH;
use merk::{Merk, Op};

use crate::{
    helpers::must_get,
    iterators::{range_bounds, MemIter, MergedIter, MerkIter},
    MerkError, clone_op,
};

pub struct StoreBase {
    /// The Merk tree which holds the key-value data.
    pub(crate) merk: Merk,

    /// Database operations from by BeginBlock, DeliverTx, and EndBlock
    /// executions, but not yet committed to the Merk store.
    ///
    /// Upon an ABCI "Commit" request, these ops will be committed to the Merk
    /// store, and this map cleared.
    pub(crate) pending_ops: BTreeMap<Vec<u8>, Op>,
}

/// Wrap a storage object inside an `Rc<RefCell<T>>` so that it can be shared as
/// an owned value, which is required by wasmer.
///
/// Adapted from Orga:
/// https://github.com/nomic-io/orga/blob/v4/src/store/share.rs#L20
///
/// A similar, thread-safe (which we don't need) implementation from Basecoin:
/// https://github.com/informalsystems/basecoin-rs/blob/c5744f4a1eac9a63ef481410e52d9fb40363b97e/src/app/store/mod.rs#L216-L218
pub struct Store(Rc<RefCell<StoreBase>>);

impl Store {
    pub fn open(path: impl AsRef<Path>) -> Result<Self, MerkError> {
        let base = StoreBase {
            merk: Merk::open(path)?,
            pending_ops: BTreeMap::new(),
        };
        Ok(Self(Rc::new(RefCell::new(base))))
    }

    pub fn share(&self) -> Self {
        Self(Rc::clone(&self.0))
    }

    fn borrow(&self) -> Ref<StoreBase> {
        self.0.borrow()
    }

    fn borrow_mut(&self) -> RefMut<StoreBase> {
        self.0.borrow_mut()
    }

    /// Derive the root hash of the blockchain state.
    pub fn root_hash(&self) -> [u8; HASH_LENGTH] {
        self.borrow().merk.root_hash()
    }

    /// Commit the pending changes to the underlying Merk store.
    /// This also writes the changes to disk, so should only be called during
    /// ABCI "Commit" requests.
    pub fn commit(&self) -> Result<(), MerkError> {
        let mut ref_mut = self.borrow_mut();

        // unlike hashmap, btreemap doesn't have a handy `drain` method
        // so we have to do this, which is very inefficient:
        let batch: Vec<_> = ref_mut.pending_ops.iter().map(clone_op).collect();
        ref_mut.pending_ops = BTreeMap::new();

        // we know the ops are sorted by keys (as they are collected from a
        // btreemap), so we skip the checking step
        unsafe { ref_mut.merk.apply_unchecked(&batch, &[]) }
    }

    /// Wrap the store into a StoreWrapper.
    ///
    /// StoreWrapper implements the Storage trait, and reads directly from the
    /// underlying Merk tree, disregarding the pending ops.
    ///
    /// StoreWrapper only supports read, and panics if a write method is invoked.
    /// It intended to be used for the "Query" ABCI request.
    pub fn wrap(&self) -> StoreWrapper {
        StoreWrapper {
            inner: self.share(),
        }
    }

    /// Wrap the store into a PendingStoreWrapper.
    ///
    /// PendingStoreWrapper implements the Storage traits. When read or write,
    /// it access the pending ops first.
    ///
    /// PendingStoreWrapper supports both read and write methods, and is
    /// intended to be used in BeginBlock/CheckTx/DeliverTx/EndBlock requests.
    pub fn pending_wrap(&self) -> PendingStoreWrapper {
        PendingStoreWrapper {
            inner: self.share(),
        }
    }
}

/// A read-only wrapper of the `Store` object, with the `cosmwasm_std::Storage`
/// trait implemented. When reading from this object, the underlying Merk store
/// is accessed, while the pending ops are ignored.
///
/// This struct is intended to be used in the ABCI "Query" request, so an
/// _immutable_ reference to the `Store` is used.
pub struct StoreWrapper {
    pub(super) inner: Store,
}

impl Storage for StoreWrapper {
    fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        must_get(&self.inner.borrow().merk, key)
    }

    fn set(&mut self, _key: &[u8], _value: &[u8]) {
        panic!("[cw-store]: `set` method invoked on read-only store wrapper");
    }

    fn remove(&mut self, _key: &[u8]) {
        panic!("[cw-store]: `remove` method invoked on read-only store wrapper");
    }

    fn range<'a>(
        &'a self,
        start: Option<&[u8]>,
        end: Option<&[u8]>,
        order: Order,
    ) -> Box<dyn Iterator<Item = Record> + 'a> {
        if let (Some(start), Some(end)) = (start, end) {
            if start > end {
                return Box::new(iter::empty());
            }
        }
        Box::new(MemIter::new(MerkIter::new(&self.inner.borrow().merk, start, end, order)))
    }
}

/// A read-and-write wrapper of the `Store` object, with the `cosmwasm_std::Storage`
/// trait implemented. When reading or writing, the `pending_ops` map is accessed.
///
/// To be used in the following ABCI requests:
/// InitChain, BeginBlock, CheckTx, DeliverTx, EndBlock
pub struct PendingStoreWrapper {
    pub(super) inner: Store,
}

impl Storage for PendingStoreWrapper {
    fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        let store = self.inner.borrow();
        let Some(op) = store.pending_ops.get(key) else {
            return must_get(&store.merk, key);
        };
        match op {
            Op::Put(value) => Some(value.clone()),
            Op::Delete => None,
        }
    }

    fn set(&mut self, key: &[u8], value: &[u8]) {
        self.inner
            .borrow_mut()
            .pending_ops
            .insert(key.to_vec(), Op::Put(value.to_vec()));
    }

    fn remove(&mut self, key: &[u8]) {
        self.inner
            .borrow_mut()
            .pending_ops
            .insert(key.to_vec(), Op::Delete);
    }

    fn range<'a>(
        &'a self,
        start: Option<&[u8]>,
        end: Option<&[u8]>,
        order: Order,
    ) -> Box<dyn Iterator<Item = Record> + 'a> {
        if let (Some(start), Some(end)) = (start, end) {
            if start > end {
                return Box::new(iter::empty());
            }
        }

        let store = self.inner.borrow();

        let base = MerkIter::new(&store.merk, start, end, order);

        let pending_raw = store.pending_ops.range(range_bounds(start, end));
        let pending: Box<dyn Iterator<Item = (&Vec<u8>, &Op)>> = match order {
            Order::Ascending => Box::new(pending_raw),
            Order::Descending => Box::new(pending_raw.rev()),
        };

        Box::new(MemIter::new(MergedIter::new(base, pending, order)))
    }
}

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use std::{env::temp_dir, time::SystemTime};

    use super::*;

    /// Open a `Store` at an autogenerated, temporary file path.
    /// Adapted from `merk::test_utils::TempMerk`:
    /// https://github.com/nomic-io/merk/blob/develop/src/test_utils/temp_merk.rs
    fn setup_test() -> Store {
        let mut path = temp_dir();
        let time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        path.push(format!("merk-temp-{time}"));

        let store = Store::open(path).unwrap();

        // add some key-values for testing
        let batch = &[
            (b"key1".to_vec(), Op::Put(b"value1".to_vec())),
            (b"key2".to_vec(), Op::Put(b"value2".to_vec())),
            (b"key3".to_vec(), Op::Put(b"value3".to_vec())),
            (b"key4".to_vec(), Op::Put(b"value4".to_vec())),
        ];
        store.borrow_mut().merk.apply(batch, &[]).unwrap();

        // add some pending ops as well
        let mut wrapper = store.pending_wrap();
        wrapper.set(b"key2", b"value23456");
        wrapper.set(b"key3333", b"value3333");
        wrapper.remove(b"key3");

        store
    }

    #[test]
    fn getting() {
        let store = setup_test();

        // read values from the read-only wrapper
        let wrapper = store.wrap();
        assert_eq!(wrapper.get(b"key1"), Some(b"value1".to_vec()));
        assert_eq!(wrapper.get(b"key2"), Some(b"value2".to_vec()));
        assert_eq!(wrapper.get(b"key3"), Some(b"value3".to_vec()));
        assert_eq!(wrapper.get(b"key3333"), None);

        // read values from the pending wrapper
        let wrapper = store.pending_wrap();
        assert_eq!(wrapper.get(b"key1"), Some(b"value1".to_vec()));
        assert_eq!(wrapper.get(b"key2"), Some(b"value23456".to_vec()));
        assert_eq!(wrapper.get(b"key3"), None);
        assert_eq!(wrapper.get(b"key3333"), Some(b"value3333".to_vec()));
    }

    #[test]
    fn committing() {
        let store = setup_test();

        store.commit().unwrap();

        let wrapper = store.wrap();
        assert_eq!(wrapper.get(b"key1"), Some(b"value1".to_vec()));
        assert_eq!(wrapper.get(b"key2"), Some(b"value23456".to_vec()));
        assert_eq!(wrapper.get(b"key3"), None);
        assert_eq!(wrapper.get(b"key3333"), Some(b"value3333".to_vec()));

        // after committing, the pending ops should have been cleared
        assert!(store.borrow().pending_ops.is_empty());
    }

    #[test]
    #[should_panic = "[cw-store]: `set` method invoked on read-only store wrapper"]
    fn illegal_set() {
        let store = setup_test();

        let mut wrapper = store.wrap();
        wrapper.set(b"should", b"panic");
    }

    #[test]
    #[should_panic = "[cw-store]: `remove` method invoked on read-only store wrapper"]
    fn illegal_remove() {
        let store = setup_test();

        let mut wrapper = store.wrap();
        wrapper.remove(b"key2");
    }

    #[cfg(feature = "iterator")]
    #[test]
    fn iterating() {
        let store = setup_test();

        let mut kv = vec![
            (b"key1".to_vec(), b"value1".to_vec()),
            (b"key2".to_vec(), b"value2".to_vec()),
            (b"key3".to_vec(), b"value3".to_vec()),
            (b"key4".to_vec(), b"value4".to_vec()),
        ];

        // iterating with no bound and in ascending order
        let items = store.wrap().range(None, None, Order::Ascending).collect::<Vec<_>>();
        assert_eq!(items, kv);

        // iterating with bounds and in ascending order
        // NOTE: lower bound is inclusive, upper bound in exclusive
        let items = store
            .wrap()
            .range(Some(b"key1234"), Some(b"key4"), Order::Ascending)
            .collect::<Vec<_>>();
        assert_eq!(items, &kv[1..3]);

        kv.reverse();

        // iterating with no bound and in descending order
        let items = store.wrap().range(None, None, Order::Descending).collect::<Vec<_>>();
        assert_eq!(items, kv);

        // iterating with bounds and in descending order
        let items = store
            .wrap()
            .range(Some(b"key1234"), Some(b"key4"), Order::Descending)
            .collect::<Vec<_>>();
        assert_eq!(items, &kv[1..3]);
    }

    #[cfg(feature = "iterator")]
    #[test]
    fn iterating_pending() {
        let store = setup_test();

        let mut kv = vec![
            (b"key1".to_vec(), b"value1".to_vec()),
            (b"key2".to_vec(), b"value23456".to_vec()),
            (b"key3333".to_vec(), b"value3333".to_vec()),
            (b"key4".to_vec(), b"value4".to_vec()),
        ];

        // iterating with no bound and in ascending order
        let items = store
            .pending_wrap()
            .range(None, None, Order::Ascending)
            .collect::<Vec<_>>();
        assert_eq!(items, kv);

        // iterating with bounds and in ascending order
        // NOTE: lower bound is inclusive, upper bound in exclusive
        let items = store
            .pending_wrap()
            .range(Some(b"key1234"), Some(b"key4"), Order::Ascending)
            .collect::<Vec<_>>();
        assert_eq!(items, &kv[1..3]);

        kv.reverse();

        // iterating with no bound and in descending order
        let items = store
            .pending_wrap()
            .range(None, None, Order::Descending)
            .collect::<Vec<_>>();
        assert_eq!(items, kv);

        // iterating with bounds and in descending order
        let items = store
            .pending_wrap()
            .range(Some(b"key1234"), Some(b"key4"), Order::Descending)
            .collect::<Vec<_>>();
        assert_eq!(items, &kv[1..3]);
    }
}
