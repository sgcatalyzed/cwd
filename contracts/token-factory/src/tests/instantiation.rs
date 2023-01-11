use cosmwasm_std::{coin, Addr};
use cw_ownable::Ownership;

use crate::{
    msg::Config,
    query,
    tests::{setup_test, OWNER},
};

#[test]
fn proper_instantiation() {
    let deps = setup_test();

    let cfg = query::config(deps.as_ref()).unwrap();
    assert_eq!(
        cfg,
        Config {
            token_creation_fee: Some(coin(12345, "ujuno"))
        },
    );

    let ownership = cw_ownable::get_ownership(deps.as_ref().storage).unwrap();
    assert_eq!(
        ownership,
        Ownership {
            owner: Some(Addr::unchecked(OWNER)),
            pending_owner: None,
            pending_expiry: None,
        },
    );
}
