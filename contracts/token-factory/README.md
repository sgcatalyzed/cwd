# cw-token-factory

Inspired by Osmosis' [x/tokenfactory][1] module, the `token-factory` contract is an implementation of the [`bank`](../bank) contract's namespace admin. It allows anyone to permissionlessly create denoms, at a fee (set by governance).

On chains with permissioned contract deployment, we expect it to be more typical for projects to reserve their own namespaces at the bank contract (at approval by governance). The `token-factory` contract is perhaps more suitable for permissionless chains.

## Terminology

Tokens created with the `token-factory` contract comes with denoms of the following format:

```plain
factory/{creator}/{nonce}
```

Where `creator` is the token creator's address, and `nonce` is an arbitrary alphanumeric string specified by the creator. See the image below for an example:

![](terminology.png)

## License

Contents of this crate at or prior to commit [`3dbd7ad`][2] are released under [GNU Affero General Public License][3] v3 or later; contents after the said commit are proprietary.

[1]: https://github.com/osmosis-labs/osmosis/tree/main/x/tokenfactory
[2]: https://github.com/steak-enjoyers/cw-sdk/commit/3dbd7ad89cfa5f5d0cf5c904b100f55a8952db3f
[3]: https://github.com/steak-enjoyers/cw-sdk/blob/3dbd7ad89cfa5f5d0cf5c904b100f55a8952db3f/LICENSE
