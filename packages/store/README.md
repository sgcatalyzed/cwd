# cw-store

Storage data structure and database backend for [CosmWasm SDK][1].

## How to use

To implement a state machine, use `SharedStore` object, which makes it sharable between threads. For example:

```rust
use cw_store::{SharedStore, Store};

struct StateMachine {
    store: SharedStore,
}

impl StateMachine {
    pub fn new(store: Store) -> Self {
        Self {
            store: SharedStore::new(store),
        }
    }
}
```

To create the state machine instance, load the store from disk:

```rust
use std::path::Path;

use cw_store::Store;

fn main() {
    let store = Store::open("./cw/data").expect("failed to open database");
    let state_machine = StateMachine::new(store);
}
```

Use `wrap`/`wrap_mut` method to create "wrappers" of the shared store. These wrappers implement the `cosmwasm_std::Storage` trait, and can be used with [cw-storage-plus][2]:

```rust
use cw_storage_plus::Item;
use tendermint_abci::{RequestDeliverTx, RequestInfo, ResponseDeliverTx, ResponseInfo};

pub const BLOCK_HEIGHI: Item<i64> = Item::new("height");

impl StateMachine {
    // for the "Info" and "Query" ABCI requests, use `wrap`
    pub fn info(&self, req: RequestInfo) -> ResponseInfo {
        let wrapper = self.store.read().wrap();

        let height = BLOCK_HEIGHT
            .load(&wrapper)
            .expect("failed to load block height");

        // ...
    }

    // for "BeginBlock", "EndBlock", "CheckTx", and "DeliverTx" ABCI requests,
    // use `wrap_mut`
    pub fn deliver_tx(&mut self, req: RequestDeliverTx) -> ResponseDeliverTx {
        let wrapper = self.store.write().wrap_mut();
        // ...
    }
}
```

For the "Commit" ABCI request, use the `commit` method:

```rust
use tendermint_abci::{RequestCommit, ResponseCommit};

impl StateMachine {
    pub fn commit(&mut self, req: RequestCommit) -> ResponseCommit {
        self.store.write().commit();
    }
}
```

The wrappers can be further wrapped in `CachedStore` and/or `PrefixedStore` objects, e.g.

```rust
use cw_store::{CachedStore, PrefixedStore};

let cached = CachedStore::new(store.wrap_mut());
let prefixed = PrefixedStore::new(store.wrap(), b"prefix");
```

## License

Contents of this crate at or prior to commit [`3dbd7ad`][3] are released under [GNU Affero General Public License][4] v3 or later; contents after the said commit are proprietary.

[1]: https://github.com/steak-enjoyers/cw-sdk
[2]: https://github.com/CosmWasm/cw-storage-plus
[3]: https://github.com/steak-enjoyers/cw-sdk/commit/3dbd7ad89cfa5f5d0cf5c904b100f55a8952db3f
[4]: https://github.com/steak-enjoyers/cw-sdk/blob/3dbd7ad89cfa5f5d0cf5c904b100f55a8952db3f/LICENSE
