use near_sdk::json_types::{Base64VecU8, U128};
use near_sdk::{serde_json::json, AccountId};
use near_sdk_sim::{
    deploy, init_simulator, to_yocto, view, ContractAccount, UserAccount, STORAGE_AMOUNT,
};

near_sdk_sim::lazy_static_include::lazy_static_include_bytes! {
    NFT_WASM_BYTES => "../res/paras_nft_contract.wasm",
    STAKING_WASM_BYTES => "../res/main.wasm",
    FT_WASM_BYTES => "../res/test_token.wasm"
}

pub const DEFAULT_GAS: u64 = near_sdk_sim::DEFAULT_GAS;
pub const NFT_ID_STR: &str = "nft";
pub const FT_ID_STR: &str = "ft";
pub const STAKING_ID_STR: &str = "staking";
pub const STORAGE_MINT_ESTIMATE: u128 = 11280000000000000000000;
pub const STORAGE_CREATE_SERIES_ESTIMATE: u128 = 8540000000000000000000;

// After calculation
pub const STORAGE_ADD_MARKET_DATA: u128 = 10590000000000000000000;
pub const STORAGE_APPROVE: u128 = 8590000000000000000000;
pub const GAS_BUY: u64 = 100 * 10u64.pow(12);

pub fn create_nft_and_mint_one(
    nft: &UserAccount,
    owner: &UserAccount,
    creator: &UserAccount,
    receiver_id_1: &UserAccount,
    receiver_id_2: &UserAccount,
) {
    owner
        .call(
            nft.account_id(),
            "nft_create_series",
            &json!({
                "token_metadata": {
                    "title": "A".repeat(200),
                    "reference": "A".repeat(59),
                    "media": "A".repeat(59),
                    "copies": 100u64,
                },
                "creator_id": creator.account_id(),
                "price": to_yocto("1").to_string(),
                "royalty": {
                    owner.account_id.clone(): 1000u32,
                    "g".repeat(64): 1000u32,
                    "h".repeat(64): 1000u32,
                    "i".repeat(64): 1000u32,
                    "j".repeat(64): 1000u32,
                    "k".repeat(64): 1000u32,
                    "l".repeat(64): 1000u32,
                    "m".repeat(64): 500u32,
                },
            })
            .to_string()
            .into_bytes(),
            DEFAULT_GAS,
            STORAGE_CREATE_SERIES_ESTIMATE * 2, //royalty
        )
        .assert_success();

    receiver_id_1
        .call(
            nft.account_id(),
            "nft_buy",
            &json!({
                "token_series_id": "1",
                "receiver_id": receiver_id_1.account_id(),
            })
            .to_string()
            .into_bytes(),
            DEFAULT_GAS,
            to_yocto("1") + STORAGE_MINT_ESTIMATE,
        )
        .assert_success();

    receiver_id_2
        .call(
            nft.account_id(),
            "nft_buy",
            &json!({
                "token_series_id": "1",
                "receiver_id": receiver_id_2.account_id(),
            })
            .to_string()
            .into_bytes(),
            DEFAULT_GAS,
            to_yocto("1") + STORAGE_MINT_ESTIMATE,
        )
        .assert_success();

    receiver_id_2
        .call(
            nft.account_id(),
            "nft_buy",
            &json!({
                "token_series_id": "1",
                "receiver_id": receiver_id_2.account_id(),
            })
            .to_string()
            .into_bytes(),
            DEFAULT_GAS,
            to_yocto("1") + STORAGE_MINT_ESTIMATE,
        )
        .assert_success();
}

pub fn init() -> (
    UserAccount,
    UserAccount,
    UserAccount,
    UserAccount,
    UserAccount,
    UserAccount,
    UserAccount,
    UserAccount,
) {
    let root = init_simulator(None);

    let alice = root.create_user(account_from(&"x"), to_yocto("100"));

    let bob = root.create_user(account_from(&"y"), to_yocto("100"));

    let chandra = root.create_user(account_from(&"z"), to_yocto("100"));

    let darmaji = root.create_user(account_from(&"n"), to_yocto("100"));

    let nft_account_id = AccountId::new_unchecked(NFT_ID_STR.to_string());
    let nft_contract = root.deploy(&NFT_WASM_BYTES, nft_account_id.clone(), STORAGE_AMOUNT);
    let staking_account_id = AccountId::new_unchecked(STAKING_ID_STR.to_string());
    let staking_contract = root.deploy(
        &STAKING_WASM_BYTES,
        staking_account_id.clone(),
        STORAGE_AMOUNT,
    );
    let ft_account_id = AccountId::new_unchecked(FT_ID_STR.to_string());
    let ft_contract = root.deploy(&FT_WASM_BYTES, ft_account_id.clone(), STORAGE_AMOUNT);
    nft_contract.call(
        nft_account_id,
        "new_default_meta",
        &json!({
            "owner_id": alice.account_id(),
            "treasury_id": alice.account_id(),
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS,
        0,
    );
    staking_contract.call(
        staking_account_id,
        "new",
        &json!({
            "owner_id": alice.account_id(),
            "nft_address": nft_account_id,
            "ft_address":ft_account_id,
            "daily_reward": 1000,
            "interval": 1000000,
            "lock_time": 1000000000
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS,
        0,
    );
    ft_contract.call(
        ft_account_id.clone(),
        "new",
        &json!({}).to_string().into_bytes(),
        DEFAULT_GAS,
        0,
    );
    (
        staking_contract,
        nft_contract,
        ft_contract,
        alice,
        bob,
        chandra,
        darmaji,
        root,
    )
}

pub fn account_o() -> AccountId {
    account_from("o")
}

pub fn account_from(s: &str) -> AccountId {
    AccountId::new_unchecked(s.repeat(64).to_string())
}
