# Messages and queries

This document includes the proposed `Msg` and `Query` API.

To emit a message, include it in the entry point function's response:

```rust
#[entry_point]
fn execute(ctx: ExecuteCtx, msg: ExecuteMsg) -> Result<Response> {
    Ok(Response::new().add_message(Msg::Execute {
        // ...
    }))
}
```

To perform a query, use `ctx.querier`:

```rust
#[entry_point]
fn execute(ctx: ExecuteCtx, msg: ExecuteMsg) -> Result<Response> {
    let res: ResponseType = ctx.querier.query(&Query::WasmSmart {
        // ...
    })?;

    Ok(Response::new())
}
```

## Messages

```rust
#[cw_serde]
enum Msg {
    // ---------------------------- governance msgs ----------------------------

    UpdateConfig {
        owner:                  Option<String>,
        bank:                   Option<String>,
        instantiate_permission: Option<InstantiatePermission>,
    },
    PinCode {
        code_hash: Hash,
    },
    UnpinCode {
        code_hash: Hash,
    },

    // ------------------------------ admin msgs -------------------------------

    UpdateAdmin {
        contract:  String,
        new_admin: String,
    },
    ClearAdmin {
        contract: String,
    },

    // -------------------------- permissionless msgs --------------------------

    Transfer {
        to:    String,
        coins: Coins,
    },
    StoreCode {
        wasm_byte_code: Binary,
    },
    Instantiate {
        code_hash: Hash,
        msg:       Binary,
        funds:     Coins,
        salt:      Option<Binary>,
        admin:     Option<String>,
    },
    Execute {
        contract: String,
        msg:      Binary,
        funds:    Coins,
    },
    Migrate {
        contract:      String,
        new_code_hash: Hash,
        msg:           Binary,
    },
    Ibc(IbcMsg),
}
```

```rust
// NOTE: We don't have light client-related messages here, because each client
// is just a wasm contract. To interact with the light client (create, update,
// upgrade, submit misbehavior...) just execute the contract directly.
#[cw_serde]
enum IbcMsg {
    ConnectionOpenInit {
        // TODO
    },
    ConnectionOpenTry {
        // TODO
    },
    ConnectionOpenAck {
        // TODO
    },
    ConnectionOpenConfirm {
        // TODO
    },
    ChannelOpenInit {
        // TODO
    },
    ChannelOpenTry {
        // TODO
    },
    ChannelOpenAck {
        // TODO
    },
    ChannelOpenConfirm {
        // TODO
    },
    ChannelCloseInit {
        // TODO
    },
    ChannelCloseConfirm {
        // TODO
    },
    SendPacket {
        // TODO
    },
    ReceivePacket {
        // TODO
    },
    Acknowledge {
        // TODO
    },
    Timeout {
        // TODO
    },
}
```

## Queries

```rust
#[cw_serde]
#[derive(QueryResponses)]
enum Query {
    #[returns(InfoResponse)]
    Info {},
    #[returns(CodeResponse)]
    Code {
        hash: Hash,
    },
    #[returns(AccountResponse)]
    Account {
        address: String,
    },
    #[returns(Vec<AccountResponse>)]
    Accounts {
        start_after: Option<String>,
        limit:       Option<u32>,
    },
    #[returns(Option<Binary>)]
    WasmRaw {
        contract: String,
        key:      Binary,
    },
    #[returns(Binary)]
    WasmSmart {
        contract: String,
        msg:      Binary,
    },
    Ibc(IbcQuery),
}
```

```rust
#[cw_serde]
#[derive(QueryResponses)]
enum IbcQuery {
    // TODO
}
```
