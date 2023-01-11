use cosmwasm_std::{Addr, Coin};
use cw_storage_plus::{Item, Map};

use crate::msg::TokenConfig;

/// The contract's configuration
pub const TOKEN_CREATION_FEE: Item<Option<Coin>> = Item::new("token_creation_fee");

/// Configuration of tokens indexed by creator address and subdenom
pub const TOKEN_CONFIGS: Map<(&Addr, &str), TokenConfig> = Map::new("tkn_cfgs");
