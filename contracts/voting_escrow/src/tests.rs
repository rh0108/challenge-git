use crate::contract::instantiate;
use anchor_token::voting_escrow::InstantiateMsg;

use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};

#[test]
fn proper_initialization() {
    let mut deps = mock_dependencies(&[]);

    let msg = InstantiateMsg {
        owner: "owner".to_string(),
        anchor_token: "anchor".to_string(),
        marketing: None,
    };

    let info = mock_info("owner", &[]);
    let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
}
