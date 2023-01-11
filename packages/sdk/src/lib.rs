//! This crate contains Type definitions and helper functions used throughout
//! cw-sdk.

//------------------------------------------------------------------------------
// Types
//------------------------------------------------------------------------------

/// Defines the genesis state, which is to be included in the Tendermint genesis
/// JSON file.
mod genesis;

/// Defines the account types.
///
/// Cw-sdk supports two types of accounts:
///
/// - base account: a.k.a. externally-owned account (EoA), is an account
///   controlled by a single public/private key pair. For cw-sdk we use
///   secp256k1 keys.
/// - contract account: an account controlled by a wasm binary code.
///
/// Each account is identified an address. The algorithms for deriving addresses
/// are described below in the `address` module.
mod account;

/// Defines the execute and query messages.
///
/// Users interact with the blockchain by sending messages to it. This includes
/// sending one or more execute messages in a transaction, which is delivered to
/// the state machine by the ABCI DeliverTx method; or sending a query message,
/// which is delivered by the ABCI Query method.
mod msg;

/// Defines the transaction type.
///
/// A transaction contains one or more execute messages, a few parameters used
/// for preventing replay attacks, and the user's signature.
mod tx;

/// Defines the required API for core contracts.
///
/// A cw-sdk chain requires at least a few "core" contracts to function, which
/// include:
///
/// - bank: manages tokens and their transfers
/// - distribution: the collection and distribution of fees and protocol revenue
/// - staking: manages the validator set and slashing on misbehviors
///
/// Cw-sdk is designed based on the idea that for each of the chain's component,
/// if if it is something we don't expect anyone wants to ever change, it should
/// be implemented as a part of the state machine. This includes transaction
/// authentication and IBC core (clients, connections, channels).
///
/// Conversely, if it is something that devs may want to customize, then it
/// should be a contract.
///
/// This crate includes the "official" implementation of some of these contracts.
/// Some of them are "core", meaning they are absolutely necessary for the chain
/// to function. This includes bank, distribution, and staking as mentioned
/// above. Others are optional, such as the gov contract (you can totally build
/// a chain where governance is a multisig, if you wish.)
mod contracts;

// export types for easy access
pub use crate::{account::*, contracts::*, genesis::*, msg::*, tx::*};

//------------------------------------------------------------------------------
// Functions
//------------------------------------------------------------------------------

/// Defines algorithms used to:
///
/// - derive account addresses for each account type
/// - resolve and validate raw addresses received from users
///
/// Each address is a 256-bit byte array, encoded in bech32, derived
/// deterministically from the account data:
///
/// - a base account's address is derived from its public key
/// - a contract account's address is derived from its label
///
/// ## Contract labels
///
/// Some special considerations must be taken for contract labels, as described
/// below.
///
/// The state machine must be programmed to ensure these labels:
///
/// - are unique: no two contract has the same label, or have labels that derive
///   the same address (i.e. hash clash);
/// - do not start with the prefix `cw1`: so that they can not be confused with
///   addresses.
///
/// ## Raw addresses
///
/// To improve developer experience, we would like that developers don't need to
/// record the address of the contracts they want to interact with; instead,
/// they can simply use the contract labels.
///
/// Here we define the concept of "raw address", which is a string that is
/// either:
///
/// - a contract address, or
/// - a contract label
///
/// We know it's an address if it starts with `cw1`, or a label otherwise.
///
/// For the convenience of users and developers, the state machine accepts raw
/// addresses instead of addresses in many instances, for example:
///
/// - when executing a contract (a user using the CLI, or a contract emitting
///   a submessage in the response) the contract address may be provided as a
///   raw address string;
/// - similarly, when querying a contract (a user using the CLI, or a contract
///   using deps.querier);
/// - when instantiating a new contract, the admin may be a raw address string
///   in SdkMsg::Instantiate.
///
/// In these case, the state machine is responsible for resolving the raw
/// address, returning the real underlying address as a cosmwasm_std::Addr.
pub mod address;

/// Defines the hash function (SHA-256) used throughout cw-sdk.
pub mod hash;

/// A few helper functions used by contracts.
pub mod helpers;
