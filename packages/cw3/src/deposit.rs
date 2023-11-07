use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, BankMsg, Coin, CosmosMsg, MessageInfo, Uint128};
use cw_utils::{must_pay, PaymentError};
use thiserror::Error;

/// Information about the deposit required to create a proposal.
#[cw_serde]
pub struct DepositInfo {
    /// The number tokens required for payment.
    pub amount: Uint128,
    /// The denom of the deposit payment.
    pub denom: String,
    /// Should failed proposals have their deposits refunded?
    pub refund_failed_proposals: bool,
}

/// Information about the deposit required to create a proposal. For
/// use in messages. To validate, transform into `DepositInfo` via
/// `into_checked()`.
#[cw_serde]
pub struct UncheckedDepositInfo {
    /// The number tokens required for payment.
    pub amount: Uint128,
    /// The denom of the deposit payment.
    pub denom: String,
    /// Should failed proposals have their deposits refunded?
    pub refund_failed_proposals: bool,
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum DepositError {
    #[error("Invalid zero deposit. Set the deposit to None to have no deposit.")]
    ZeroDeposit {},

    #[error("Invalid cw20")]
    InvalidCw20 {},

    #[error("{0}")]
    Payment(#[from] PaymentError),

    #[error("Invalid native deposit amount")]
    InvalidDeposit {},
}

impl UncheckedDepositInfo {
    /// Checks deposit info.
    pub fn into_checked(self) -> Result<DepositInfo, DepositError> {
        if self.amount.is_zero() {
            Err(DepositError::ZeroDeposit {})
        } else {
            Ok(DepositInfo {
                amount: self.amount,
                denom: self.denom,
                refund_failed_proposals: self.refund_failed_proposals,
            })
        }
    }
}

impl DepositInfo {
    pub fn check_native_deposit_paid(&self, info: &MessageInfo) -> Result<(), DepositError> {
        let paid = must_pay(info, &self.denom)?;
        if paid != self.amount {
            Err(DepositError::InvalidDeposit {})
        } else {
            Ok(())
        }
    }

    pub fn get_return_deposit_message(&self, depositor: &Addr) -> CosmosMsg {
        BankMsg::Send {
            to_address: depositor.to_string(),
            amount: vec![Coin {
                amount: self.amount,
                denom: self.denom.clone(),
            }],
        }
        .into()
    }
}
