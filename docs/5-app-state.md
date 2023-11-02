# App state

The state of a CWD chain (of "application" in the Cosmos SDK lingo) is a Merkle forest, i.e. multiple Merkle trees:

| Tree     | Prefix[^1]                | Description                                                                     |
| -------- | ------------------------- | ------------------------------------------------------------------------------- |
| Global   | `b"g"`                    | The chain's global state, such as chain ID, last committed height, config, etc. |
| Code     | `b"w"`                    | Stored Wasm bytecodes                                                           |
| Account  | `b"a"`                    | Metadata of instantiated contracts                                              |
| IBC      | `b"i"`                    | IBC connection/channel states                                                   |
| Contract | `b"c"` + contract address | Internal state of each contract                                                 |

The root of each Contract tree is included in its metadata in the Acccount tree.

The application state root is simply:

```plain
root := hash(global_root | code_root | account_root | ibc_root)
```

## Global tree

```rust
const CHAIN_ID:               Item<String>;
const OWNER:                  Item<Addr>;
const BANK:                   Item<Addr>;
const INSTANTIATE_PERMISSION: Item<InstantiatePermission>;
const LAST_COMMITTED_HEIGHT:  Item<u64>;
```

```rust
#[cw_serde]
enum InstantiatePermission {
    /// Anyone can instantiate contracts
    Everybody,
    /// Only the chain owner can instantiate contracts
    Nobody,
    /// Only the owner and addresses in the whitelist can instantiate contracts
    Whitelist(BTreeSet<String>),
}
```

## Code tree

```rust
const CODES:        Map<&Hash, Binary>;
const PINNED_CODES: Set<&Hash>;
```

## Account tree

```rust
const ACCOUNTS: Map<&Addr, Account>;
```

```rust
#[cw_serde]
struct Account {
    pub code_hash: Hash,
    pub root_hash: Hash,
    pub admin:     Option<String>,
}
```

## IBC tree

```rust
// TODO
```

[^1]: In the underlying physical database, each key is prefixed with this to distinguish which tree it belongs to.
