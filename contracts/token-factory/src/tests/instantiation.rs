use cosmwasm_std::Addr;
use cw_ownable::Ownership;

use crate::{
    query,
    tests::{fee, setup_test, OWNER},
};

#[test]
fn proper_instantiation() {
    let deps = setup_test();

    let token_creation_fee = query::token_creation_fee(deps.as_ref()).unwrap();
    assert_eq!(token_creation_fee, Some(fee()));

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
