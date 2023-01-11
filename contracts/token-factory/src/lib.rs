pub mod contract;
pub mod error;
pub mod execute;
pub mod helpers;
pub mod msg;
pub mod query;
pub mod state;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod integration_tests;

/// The bank contract's label
pub const BANK: &str = "bank";

/// The namespace that the token factory contract must be assigned as admin at
/// the bank contract.
pub const NAMESPACE: &str = "factory";
