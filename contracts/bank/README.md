# cw-bank

The `bank` contract handles minting, burning, and transfers of fungible tokens.

## Namespaces

The contract allows creation of token **namespaces**, and appointing third party accounts as admins to namespaces. The admin has the power to mint, burn, and force-transfer tokens under the namespace, as well as configuring an "after send hook" which is invoked every time a token under the namespace is transferred.

See the comments in the [`denom`](./src/denom/mod.rs#L1-L23) module for further explanations on the namespace semantics.

See the [`token-factory`](../token-factory/) contract for an example implementation of namespace admin contracts.

## License

Contents of this crate at or prior to commit [`3dbd7ad`][1] are released under [GNU Affero General Public License][2] v3 or later; contents after the said commit are proprietary.

[1]: https://github.com/steak-enjoyers/cw-sdk/commit/3dbd7ad89cfa5f5d0cf5c904b100f55a8952db3f
[2]: https://github.com/steak-enjoyers/cw-sdk/blob/3dbd7ad89cfa5f5d0cf5c904b100f55a8952db3f/LICENSE
