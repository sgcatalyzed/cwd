# Entry points

This document lists all entry points that a contract may implement.

```rust
#[entry_point]
fn instantiate(ctx: InstantiateCtx, msg: InstantiateMsg) -> Result<Response>;

#[entry_point]
fn execute(ctx: ExecuteCtx, msg: ExecuteMsg) -> Result<Response>;

#[entry_point]
fn reply(ctx: ReplyCtx, reply: Reply) -> Result<Response>;

#[entry_point]
fn query(ctx: QueryCtx, msg: QueryMsg) -> Result<Response>;

#[entry_point]
fn migrate(ctx: MigrateCtx, msg: MigrateMsg) -> Result<Response>;

#[entry_point]
fn before_tx(ctx: BeforeTxCtx, tx: Tx) -> Result<Response>;

#[entry_point]
fn after_tx(ctx: AfterTxCtx) -> Result<Response>;

#[entry_point]
fn receive(ctx: ReceiveCtx, from: String, coins: Coins) -> Result<Response>;

#[entry_point]
fn transfer(
    ctx:   TransferCtx,
    from:  String,
    to:    String,
    coins: Coins,
) -> Result<Response>;

#[entry_point]
fn ibc_channel_open_init(
    ctx:     IbcChannelOpenInitCtx,
    channel: IbcChannel,
) -> Result<Response>;

#[entry_point]
fn ibc_channel_open_try(
    ctx:                  IbcChannelOpenTryCtx,
    channel:              IbcChannel,
    counterparty_version: String,
) -> Result<Response>;

#[entry_point]
fn ibc_channel_open_connect(
    ctx:                  IbcChannelOpenConnectCtx,
    channel:              IbcChannel,
    counterparty_version: String,
) -> Result<Response>;

#[entry_point]
fn ibc_channel_close_init(
    ctx:     IbcChannelCloseInitCtx,
    channel: IbcChannel,
) -> Result<Response>;

#[entry_point]
fn ibc_channel_close_confirm(
    ctx:     IbcChannelCloseConfirmCtx,
    channel: IbcChannel,
) -> Result<Response>;

#[entry_point]
fn ibc_packet_receive(
    ctx:    IbcPacketReceiveCtx,
    packet: IbcPacket,
) -> Result<Response>;

#[entry_point]
fn ibc_packet_ack(
  ctx:    IbcPacketAckCtx,
  ack:    IbcAcknowledgement,
  packet: IbcPacket,
) -> Result<Response>;

#[entry_point]
fn ibc_packet_timeout(
    ctx:    IbcPacketReceiveCtx,
    packet: IbcPacket,
) -> Result<Response>;

// TODO: add IBC light client entry points
```
