use cosmwasm_schema::cw_serde;
use cosmwasm_std::Coin;

pub mod bank {
    use super::*;

    #[cw_serde]
    pub enum SudoMsg {
        /// Forcibly transfer coins between two accounts.
        ///
        /// Callable by the state machine when handling gas fee payments and
        /// funds attached to messages.
        Transfer {
            from: String,
            to: String,
            coins: Vec<Coin>,
        },
    }
}
