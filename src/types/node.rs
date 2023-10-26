use {
    crate::types::{Hash, Nibble},
    blake3::Hasher,
    cosmwasm_schema::cw_serde,
};

#[cw_serde]
pub enum Node {
    Internal(InternalNode),
    Leaf(LeafNode),
}

impl Node {
    pub fn hash(&self) -> Hash {
        match self {
            Node::Internal(internal_node) => internal_node.hash(),
            Node::Leaf(leaf_node) => leaf_node.hash(),
        }
    }
}

#[cw_serde]
#[derive(Eq)]
pub struct Child {
    pub index: Nibble,
    pub version: u64,
    pub hash: Hash,
}

// Ideally we want to usd a map type such as BTreeMap. Unfortunately, CosmWasm
// doesn't support serialization for map types:
// https://github.com/CosmWasm/serde-json-wasm/issues/41
#[cw_serde]
pub struct Children(Vec<Child>);

impl From<Vec<Child>> for Children {
    fn from(vec: Vec<Child>) -> Self {
        Self(vec)
    }
}

impl AsRef<[Child]> for Children {
    fn as_ref(&self) -> &[Child] {
        self.0.as_slice()
    }
}

impl<'a> IntoIterator for &'a Children {
    type Item = &'a Child;
    type IntoIter = std::slice::Iter<'a, Child>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.as_slice().iter()
    }
}

impl Children {
    pub fn count(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn get(&self, index: Nibble) -> Option<&Child> {
        self.0
            .iter()
            .find(|child| child.index == index)
    }

    /// If there is exactly one child, return a reference to this child.
    /// Otherwise (no child or more than one children), panic.
    pub fn get_only(&self) -> &Child {
        if self.0.len() != 1 {
            panic!("not exactly one child");
        }

        &self.0[0]
    }

    pub fn insert(&mut self, new_child: Child) {
        for (pos, child) in self.0.iter().enumerate() {
            if child.index == new_child.index {
                self.0[pos] = new_child;
                return;
            }

            if child.index > new_child.index {
                self.0.insert(pos, new_child);
                return;
            }
        }

        self.0.push(new_child);
    }

    pub fn remove(&mut self, index: Nibble) {
        let Some(pos) = self.0.iter().position(|child| child.index == index) else {
            panic!("child not found with index {index}");
        };

        self.0.remove(pos);
    }
}

#[cw_serde]
pub struct InternalNode {
    pub children: Children,
    // different from Ethereum's Patricia Merkle Tree, in our case the internal
    // node itself may also have values
    pub kv: Option<LeafNode>,
}

impl InternalNode {
    pub fn new(kv: LeafNode, children: Vec<Child>) -> Self {
        Self {
            kv: Some(kv),
            children: children.into(),
        }
    }

    pub fn new_empty(children: Vec<Child>) -> Self {
        Self {
            kv: None,
            children: children.into(),
        }
    }

    // We define the hash of an internal node as
    //
    // hash(childA.index || childA.hash || ... || childZ.index || childZ.hash)
    //
    // where || means byte concatenation, and child{A..Z} are children that
    // exist, in ascending order. That is, non-existing children are not part
    // of the preimage.
    pub fn hash(&self) -> Hash {
        let mut hasher = Hasher::new();

        // hash the children
        for child in &self.children {
            hasher.update(&[child.index.byte()]);
            hasher.update(child.hash.as_bytes());
        }

        // has the internal node's own KV (if exists)
        if let Some(LeafNode { key, value }) = &self.kv {
            hasher.update(key.as_bytes());
            hasher.update(value.as_bytes());
        }

        hasher.finalize().into()
    }
}

#[cw_serde]
pub struct LeafNode {
    pub key: String,
    pub value: String,
}

impl LeafNode {
    pub fn new(key: String, value: String) -> Self {
        Self {
            key,
            value,
        }
    }

    /// We define the hash of a leaf node as:
    ///
    /// hash(len(key) || key || value)
    ///
    /// where || means byte concatenation, and len() returns a 16-bit unsigned
    /// integer in big endian encoding.
    ///
    /// The length prefix is necessary, because otherwise we won't be able to
    /// differentiate, for example, these two:
    ///
    /// | key       | value    |
    /// | --------- | -------- |
    /// | `b"foo"`  | `b"bar"` |
    /// | `b"foob"` | `b"ar"`  |
    pub fn hash(&self) -> Hash {
        let mut hasher = Hasher::new();
        hasher.update((self.key.as_bytes().len() as u16).to_be_bytes().as_slice());
        hasher.update(self.key.as_bytes());
        hasher.update(self.value.as_bytes());
        hasher.finalize().into()
    }
}
