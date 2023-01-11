mod creating;
mod fee;
mod hook;
mod instantiation;
mod minting;

use cosmwasm_std::{
    coin,
    testing::{mock_dependencies, mock_info, MockApi, MockQuerier, MockStorage},
    Empty, OwnedDeps, Coin,
};

use crate::execute;

const OWNER: &str = "larry";
const DENOM: &str = "factory/larry/uastro";

fn fee() -> Coin {
    coin(12345, "ujuno")
}

fn setup_test() -> OwnedDeps<MockStorage, MockApi, MockQuerier, Empty> {
    let mut deps = mock_dependencies();

    execute::init(deps.as_mut(), OWNER, Some(fee())).unwrap();

    execute::create_token(
        deps.as_mut(),
        mock_info("larry", &[fee()]),
        "uastro".into(),
        "jake".into(),
        Some("pumpkin".into()),
    )
    .unwrap();

    deps
}
