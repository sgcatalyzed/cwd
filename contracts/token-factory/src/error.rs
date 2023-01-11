use cosmwasm_std::{Coin, StdError, Uint128};
use cw_bank::denom::DenomError;
use cw_utils::PaymentError;
use thiserror::Error;

use crate::NAMESPACE;

#[derive(Debug, Error)]
#[cfg_attr(any(test, feature = "library"), derive(PartialEq))]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    Address(#[from] cw_sdk::address::AddressError),

    #[error("{0}")]
    Ownership(#[from] cw_ownable::OwnershipError),

    #[error("{0}")]
    Payment(#[from] PaymentError),

    #[error("{0}")]
    Denom(#[from] DenomError),

    #[error("the contract has no coins to transfer")]
    NoBalance,

    #[error("sender is not the bank contract")]
    NotBank,

    #[error("sender is not the admin of denom {denom}")]
    NotTokenAdmin {
        denom: String,
    },

    #[error("incorrect fee amount: expected {expected}, received {received}")]
    IncorrectFee {
        expected: Coin,
        received: Uint128,
    },

    #[error("invalid denom {denom}: must be of format `factory/{{creator}}/{{nonce}}`")]
    InvalidDenomFormat {
        denom: String,
    },

    #[error("invalid denom {denom}: namespace must be {NAMESPACE}")]
    InvalidDenomNamespace {
        denom: String,
    },

    #[error("token of denom {denom} already exists")]
    TokenExists {
        denom: String,
    },

    #[error("token of denom {denom} does not exist")]
    TokenNotFound {
        denom: String,
    },
}

impl ContractError {
    pub fn not_token_admin(denom: impl Into<String>) -> Self {
        Self::NotTokenAdmin {
            denom: denom.into(),
        }
    }

    pub fn incorrect_fee(expected: Coin, received: Uint128) -> Self {
        Self::IncorrectFee {
            expected,
            received,
        }
    }

    pub fn incorrect_denom_format(denom: impl Into<String>) -> Self {
        Self::InvalidDenomFormat {
            denom: denom.into(),
        }
    }

    pub fn incorrect_denom_namespace(denom: impl Into<String>) -> Self {
        Self::InvalidDenomNamespace {
            denom: denom.into(),
        }
    }

    pub fn token_exists(denom: impl Into<String>) -> Self {
        Self::TokenExists {
            denom: denom.into(),
        }
    }

    pub fn token_not_found(denom: impl Into<String>) -> Self {
        Self::TokenNotFound {
            denom: denom.into(),
        }
    }
}
