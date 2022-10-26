use near_contract_standards::non_fungible_token::Token;
use near_sdk::{
    json_types::{U128, U64},
    serde::{Deserialize, Serialize},
    serde_json::json,
    AccountId, Gas,
};
use near_sdk_sim::{call, deploy, init_simulator, to_yocto, view, ContractAccount, UserAccount};

use crate::utils::{
    account_o, create_nft_and_mint_one, init, DEFAULT_GAS, GAS_BUY, STORAGE_ADD_MARKET_DATA,
    STORAGE_APPROVE,
};

mod utils;

#[test]
fn test_init() {
    let (staking, nft, _, _, _, _, _, _) = init();
    let owner = staking
        .view(
            staking.account_id(),
            "get_owner",
            &json!({}).to_string().into_bytes(),
        )
        .unwrap_json();
    println!("ownerrrrrrrr: {:?} \n\n", owner);
}
