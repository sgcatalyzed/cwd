# Transaction lifecycle

A CWD transaction (tx) data structure is as follows:

```rust
#[cw_serde]
struct Tx {
    pub sender:     String,
    pub msgs:       Vec<cw_std::Msg>,
    pub credential: Binary,
}
```

On receiving the tx, the CWD state machine will call the `before_tx` entry point of contract indicated by the `sender` address. The contract is supposed to authenticate the tx here (i.e. assert that whoever sent it is authorized to do so) based on the given `credential`.

Notice that the tx lacks things such as `chain_id` or `sequence` which are found in Cosmos SDK txs. These data are necessary to prevent replay attacks. How do we solve this?

Remember, every account in CWD is a smart contract. We can thus implement replay guard in the contract. Here's an example:

```rust
use cw_schema::cw_serde;
use cw_std::{to_binary, BeforeTxCtx, Binary, Msg, Response};
use cw_storage::Item;

// this account contract mimicks the behavior of an Externally Owned Account (EOA),
// that is, an account that is controlled by a single private key.
// to do this, the contract records the corresponding public key
const PUBKEY: Item<Binary> = Item::new("pubkey");

// the contract maintains a sequence number in its state.
// should be initialized to 0 during instantiation
const SEQUENCE: Item<u64> = Item::new("sequence");

// this is the data that the tx sender is expected to sign with the privkey.
// it includes the chain ID as well as the account sequence number, for replay
// protection
#[cw_serde]
struct SignDoc {
    pub sender:   String,
    pub msgs:     Vec<Msg>,
    pub chain_id: String,
    pub sequence: u64,
}

#[entry_point]
fn before_tx(ctx: BeforeTxCtx, tx: Tx) -> Result<Response> {
    let pubkey = PUBKEY.load(ctx.storage)?;
    let mut sequence = SEQUENCE.load(ctx.storage)?;

    // sequence should be incremented
    sequence += 1;

    // build the sign doc and serialize it into binary
    let sign_doc_bytes = to_binary(&SignDoc {
        sender:   tx.sender,
        msgs:     tx.msgs,
        chain_id: ctx.block.chain_id,
        sequence: new_seq,
    })?;

    // verify the signature
    // use any cryptography library of your choice, for example secp256k1:
    // https://crates.io/crates/secp256k1
    verify_signature(&sign_doc_bytes, &pubkey, &tx.credential)?;

    // save the incremented sequence
    SEQUENCE.save(ctx.storage, &sequence)?;

    Ok(Response::new())
}
```
