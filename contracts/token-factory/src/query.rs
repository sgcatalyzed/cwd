use cosmwasm_std::{Addr, Coin, Deps, StdResult};
use cw_paginate::paginate_map;
use cw_storage_plus::Bound;

use crate::{
    error::ContractError,
    helpers::parse_denom,
    msg::TokenResponse,
    state::{TOKEN_CONFIGS, TOKEN_CREATION_FEE},
    NAMESPACE,
};

pub fn token_creation_fee(deps: Deps) -> StdResult<Option<Coin>> {
    TOKEN_CREATION_FEE.load(deps.storage)
}

pub fn token(deps: Deps, denom: String) -> Result<TokenResponse, ContractError> {
    let (creator, nonce) = parse_denom(deps.api, &denom)?;
    let cfg = TOKEN_CONFIGS.load(deps.storage, (&creator, &nonce))?;
    Ok(TokenResponse {
        denom,
        admin: cfg.admin.map(String::from),
        after_transfer_hook: cfg.after_transfer_hook.map(String::from),
    })
}

pub fn tokens(
    deps: Deps,
    start_after: Option<String>,
    limit: Option<u32>,
) -> Result<Vec<TokenResponse>, ContractError> {
    // a little hack to circumvent rust borrow check
    let (creator, nonce): (Addr, String);

    let start = match start_after {
        Some(s) => {
            (creator, nonce) = parse_denom(deps.api, &s)?;
            Some(Bound::exclusive((&creator, nonce.as_str())))
        },
        None => None,
    };

    paginate_map(&TOKEN_CONFIGS, deps.storage, start, limit, |(creator, nonce), cfg| {
        Ok(TokenResponse {
            denom: format!("{NAMESPACE}/{creator}/{nonce}"),
            admin: cfg.admin.map(String::from),
            after_transfer_hook: cfg.after_transfer_hook.map(String::from),
        })
    })
}
