use cosmwasm_std::{
    to_binary, Addr, BlockInfo, Coin, Deps, DepsMut, Env, MessageInfo, Response, Uint128, WasmMsg,
};
use cw_bank::{denom::Denom, msg as bank};
use cw_ownable::{assert_owner, Action as OwnershipAction};
use cw_sdk::{
    address,
    helpers::{stringify_coins, stringify_option, validate_optional_addr},
};
use cw_utils::must_pay;

use crate::{
    error::ContractError,
    helpers::parse_denom,
    msg::TokenConfig,
    state::{TOKEN_CONFIGS, TOKEN_CREATION_FEE},
    BANK,
    NAMESPACE,
};

pub fn init(
    deps: DepsMut,
    owner: &str,
    token_creation_fee: Option<Coin>,
) -> Result<Response, ContractError> {
    cw_ownable::initialize_owner(deps.storage, deps.api, Some(owner))?;

    TOKEN_CREATION_FEE.save(deps.storage, &token_creation_fee)?;

    Ok(Response::default())
}

pub fn update_ownership(
    deps: DepsMut,
    block: &BlockInfo,
    sender: &Addr,
    action: OwnershipAction,
) -> Result<Response, ContractError> {
    let ownership = cw_ownable::update_ownership(deps, block, sender, action)?;

    Ok(Response::new()
        .add_attribute("action", "token-factory/update_ownership")
        .add_attributes(ownership.into_attributes()))
}

pub fn update_fee(
    deps: DepsMut,
    info: MessageInfo,
    token_creation_fee: Option<Coin>,
) -> Result<Response, ContractError> {
    cw_ownable::assert_owner(deps.as_ref().storage, &info.sender)?;

    TOKEN_CREATION_FEE.save(deps.storage, &token_creation_fee)?;

    Ok(Response::new()
        .add_attribute("action", "token-factory/update_fee")
        .add_attribute("new_fee", stringify_option(token_creation_fee)))
}

pub fn withdraw_fee(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    to: Option<String>,
) -> Result<Response, ContractError> {
    assert_owner(deps.as_ref().storage, &info.sender)?;

    let coins: Vec<Coin> = deps.querier.query_wasm_smart(
        BANK,
        &bank::QueryMsg::Balances {
            address: env.contract.address.to_string(),
            start_after: None,
            limit: None,
        },
    )?;

    if coins.is_empty() {
        return Err(ContractError::NoBalance);
    }

    let to = to.unwrap_or_else(|| info.sender.into());

    Ok(Response::new()
        .add_attribute("action", "token-factory/withdraw_fee")
        .add_attribute("to", &to)
        .add_attribute("coins", stringify_coins(&coins))
        .add_message(WasmMsg::Execute {
            contract_addr: BANK.into(),
            msg: to_binary(&bank::ExecuteMsg::Send {
                to,
                coins,
            })?,
            funds: vec![],
        }))
}

pub fn create_token(
    deps: DepsMut,
    info: MessageInfo,
    nonce: String,
    admin: String,
    after_transfer_hook: Option<String>,
) -> Result<Response, ContractError> {
    let fee = TOKEN_CREATION_FEE.load(deps.storage)?;

    if let Some(fee) = fee {
        let received_amount = must_pay(&info, &fee.denom)?;
        if received_amount != fee.amount {
            return Err(ContractError::incorrect_fee(fee, received_amount));
        }
    }

    let denom = format!("{NAMESPACE}/{}/{nonce}", &info.sender);
    Denom::validate(&denom)?;

    TOKEN_CONFIGS.update(deps.storage, (&info.sender, &nonce), |opt| {
        if opt.is_some() {
            return Err(ContractError::token_exists(&denom));
        }
        Ok(TokenConfig {
            admin: Some(deps.api.addr_validate(&admin)?),
            after_transfer_hook: validate_optional_addr(deps.api, after_transfer_hook.as_ref())?,
        })
    })?;

    Ok(Response::new()
        .add_attribute("action", "token-factory/create_token")
        .add_attribute("denom", denom)
        .add_attribute("admin", admin)
        .add_attribute("after_transfer_hook", stringify_option(after_transfer_hook)))
}

pub fn update_token(
    deps: DepsMut,
    info: MessageInfo,
    denom: String,
    admin: Option<String>,
    after_transfer_hook: Option<String>,
) -> Result<Response, ContractError> {
    let (creator, nonce) = assert_denom_admin(deps.as_ref(), &denom, &info.sender)?;

    TOKEN_CONFIGS.update(deps.storage, (&creator, &nonce), |opt| -> Result<_, ContractError> {
        let mut token_cfg = opt.ok_or_else(|| ContractError::token_not_found(&denom))?;
        token_cfg.admin = validate_optional_addr(deps.api, admin.as_ref())?;
        token_cfg.after_transfer_hook = validate_optional_addr(deps.api,after_transfer_hook.as_ref())?;
        Ok(token_cfg)
    })?;

    Ok(Response::new()
        .add_attribute("action", "token-factory/update_token")
        .add_attribute("denom", denom)
        .add_attribute("admin", stringify_option(admin))
        .add_attribute("after_transfer_hook", stringify_option(after_transfer_hook)))
}

pub fn mint(
    deps: DepsMut,
    info: MessageInfo,
    to: String,
    denom: String,
    amount: Uint128,
) -> Result<Response, ContractError> {
    assert_denom_admin(deps.as_ref(), &denom, &info.sender)?;

    Ok(Response::new()
        .add_attribute("action", "token-factory/mint")
        .add_attribute("to", &to)
        .add_attribute("coin", format!("{amount}{denom}"))
        .add_message(WasmMsg::Execute {
            contract_addr: BANK.into(),
            msg: to_binary(&bank::ExecuteMsg::Mint {
                to,
                denom,
                amount,
            })?,
            funds: vec![],
        }))
}

pub fn burn(
    deps: DepsMut,
    info: MessageInfo,
    from: String,
    denom: String,
    amount: Uint128,
) -> Result<Response, ContractError> {
    assert_denom_admin(deps.as_ref(), &denom, &info.sender)?;

    Ok(Response::new()
        .add_attribute("action", "token-factory/burn")
        .add_attribute("from", &from)
        .add_attribute("coin", format!("{amount}{denom}"))
        .add_message(WasmMsg::Execute {
            contract_addr: BANK.into(),
            msg: to_binary(&bank::ExecuteMsg::Burn {
                from,
                denom,
                amount,
            })?,
            funds: vec![],
        }))
}

pub fn force_transfer(
    deps: DepsMut,
    info: MessageInfo,
    from: String,
    to: String,
    denom: String,
    amount: Uint128,
) -> Result<Response, ContractError> {
    assert_denom_admin(deps.as_ref(), &denom, &info.sender)?;

    Ok(Response::new()
        .add_attribute("action", "token-factory/burn")
        .add_attribute("from", &from)
        .add_attribute("coin", format!("{amount}{denom}"))
        .add_message(WasmMsg::Execute {
            contract_addr: BANK.into(),
            msg: to_binary(&bank::ExecuteMsg::ForceTransfer {
                from,
                to,
                denom,
                amount,
            })?,
            funds: vec![],
        }))
}

pub fn after_transfer(
    deps: DepsMut,
    info: MessageInfo,
    from: String,
    to: String,
    denom: String,
    amount: Uint128,
) -> Result<Response, ContractError> {
    assert_sender_bank(&info.sender)?;

    let (creator_addr, nonce) = parse_denom(deps.api, &denom)?;
    let token_cfg = TOKEN_CONFIGS.load(deps.storage, (&creator_addr, &nonce))?;

    // do nothing if `after_transfer_hook` is not set for this denom
    let Some(after_transfer_hook) = token_cfg.after_transfer_hook else {
        return Ok(Response::default());
    };

    Ok(Response::new()
        .add_attribute("action", "token-factory/after_transfer")
        .add_attribute("from", &from)
        .add_attribute("to", &to)
        .add_attribute("coin", format!("{amount}{denom}"))
        .add_message(WasmMsg::Execute {
            contract_addr: after_transfer_hook.into(),
            msg: to_binary(&bank::HookMsg::AfterTransfer {
                from,
                to,
                denom,
                amount,
            })?,
            funds: vec![],
        }))
}

/// Assert that the sender is the bank contract.
fn assert_sender_bank(sender: &Addr) -> Result<(), ContractError> {
    let bank = address::derive_from_label(BANK)?;

    if *sender != bank {
        return Err(ContractError::NotBank);
    }

    Ok(())
}

/// Assert that sender is the denom's current admin. Return the denom's creator
/// and nonce.
fn assert_denom_admin(
    deps: Deps,
    denom: &str,
    sender: &Addr,
) -> Result<(Addr, String), ContractError> {
    let (creator, nonce) = parse_denom(deps.api, denom)?;

    let Some(token_cfg) = TOKEN_CONFIGS.may_load(deps.storage, (&creator, &nonce))? else {
        return Err(ContractError::token_not_found(denom));
    };

    let Some(admin) = token_cfg.admin else {
        return Err(ContractError::not_token_admin(denom));
    };

    if *sender != admin {
        return Err(ContractError::not_token_admin(denom));
    }

    Ok((creator, nonce))
}
