mod cache;
mod helpers;
pub mod iterators;
pub mod prefix;
mod share;
mod store;

pub use crate::cache::Cached;
pub use crate::share::Shared;
pub use crate::store::{PendingStoreWrapper, Store, StoreBase, StoreWrapper};

pub use merk::Error as MerkError;

// merk::Op doesn't implement Clone, so we have to do this:
use merk::Op;

fn clone_op((key, op): (&Vec<u8>, &Op)) -> (Vec<u8>, Op) {
    let cloned_op = match op {
        Op::Put(value) => Op::Put(value.clone()),
        Op::Delete => Op::Delete,
    };

    (key.clone(), cloned_op)
}
