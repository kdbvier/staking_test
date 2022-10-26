use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap, LookupSet, UnorderedMap, UnorderedSet};
use near_sdk::json_types::{U128, U64};
use near_sdk::{
    assert_one_yocto, env, ext_contract, near_bindgen, AccountId, Balance, CryptoHash,
    PanicOnDefault, Promise, PromiseOrValue, PromiseResult,
};

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Token {
    pub address: AccountId,
    pub token_id: String,
    pub claimed_amount: U128,
    pub unclaimed_amount: U128,
    pub claimed_timestamp: u64,
    pub create_unstake_timestamp: u64,
    pub last_timestamp: u64,
}
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    //contract owner
    pub owner_id: AccountId,

    //keeps track of all the token IDs for a given account
    pub tokens_per_owner: LookupMap<AccountId, UnorderedSet<String>>,

    //keeps track of the token struct for a given token ID
    pub tokens_by_id: LookupMap<String, Token>,

    pub storage_deposits: LookupMap<AccountId, Balance>,
    pub nft_address: AccountId,
    pub ft_address: AccountId,
    pub daily_reward: U128,
    pub interval: u64,
    pub lock_time: u64,
    pub enabled: bool,
}

#[derive(BorshSerialize)]
pub enum StorageKey {
    TokensPerOwner,
    TokensById,
    ConfigData,
    StorageDeposits,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(
        owner_id: AccountId,
        nft_address: AccountId,
        ft_address: AccountId,
        daily_reward: U128,
        interval: u64,
        lock_time: u64,
    ) -> Self {
        let this = Self {
            tokens_per_owner: LookupMap::new(StorageKey::TokensPerOwner.try_to_vec().unwrap()),
            tokens_by_id: LookupMap::new(StorageKey::TokensById.try_to_vec().unwrap()),
            owner_id,
            storage_deposits: LookupMap::new(StorageKey::StorageDeposits.try_to_vec().unwrap()),
            nft_address,
            ft_address,
            daily_reward,
            interval,
            lock_time,
            enabled: false,
        };

        this
    }
    #[payable]
    pub fn update_owner(&mut self, owner_id: AccountId) {
        assert_one_yocto();
        assert_eq!(
            env::predecessor_account_id(),
            self.owner_id,
            "Marble: Owner only"
        );
        self.owner_id = owner_id;
    }

    #[payable]
    pub fn update_enable(&mut self, enabled: bool) {
        assert_one_yocto();
        assert_eq!(
            env::predecessor_account_id(),
            self.owner_id,
            "Marble: Owner only"
        );
        self.enabled = enabled;
    }

    #[payable]
    fn _internal_receive_nft(
        &mut self,
        _nft_contract_id: AccountId,
        _previous_owner_id: AccountId,
        _token_id: String,
    ) {
    }

    pub fn get_owner(&self) -> AccountId {
        self.owner_id.clone()
    }

    pub fn get_enable_status(&self) -> bool {
        self.enabled.clone()
    }
    pub fn get_supply_by_owner_id(&self, account_id: AccountId) -> U64 {
        self.tokens_per_owner
            .get(&account_id)
            .map_or(0, |by_owner_id| by_owner_id.len())
            .into()
    }
    // // Storage
    // #[payable]
    // pub fn storage_deposit(&mut self, account_id: Option<AccountId>) {
    //     let storage_account_id = account_id
    //         .map(|a| a.into())
    //         .unwrap_or_else(env::predecessor_account_id);
    //     let deposit = env::attached_deposit();
    //     assert!(
    //         deposit >= STORAGE_ADD_STAKING_DATA,
    //         "Requires minimum deposit of {}",
    //         STORAGE_ADD_STAKING_DATA
    //     );

    //     let mut balance: u128 = self.storage_deposits.get(&storage_account_id).unwrap_or(0);
    //     balance += deposit;
    //     self.storage_deposits.insert(&storage_account_id, &balance);
    // }

    // #[payable]
    // pub fn storage_withdraw(&mut self) {
    //     assert_one_yocto();
    //     let owner_id = env::predecessor_account_id();
    //     let mut amount = self.storage_deposits.remove(&owner_id).unwrap_or(0);
    //     let market_data_owner = self.tokens_per_owner.get(&owner_id);
    //     let len = market_data_owner.map(|s| s.len()).unwrap_or_default();
    //     let diff = u128::from(len) * STORAGE_ADD_STAKING_DATA;
    //     amount -= diff;
    //     if amount > 0 {
    //         Promise::new(owner_id.clone()).transfer(amount);
    //     }
    //     if diff > 0 {
    //         self.storage_deposits.insert(&owner_id, &diff);
    //     }
    // }

    // pub fn storage_minimum_balance(&self) -> U128 {
    //     U128(STORAGE_ADD_STAKING_DATA)
    // }

    // pub fn storage_balance_of(&self, account_id: AccountId) -> U128 {
    //     self.storage_deposits.get(&account_id).unwrap_or(0).into()
    // }
}

/*
 * the rest of this file sets up unit tests
 * to run these, the command will be:
 * cargo test --package rust-template -- --nocapture
 * Note: 'rust-template' comes from Cargo.toml's 'name' key
 */

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::{testing_env, AccountId};

    // part of writing unit tests is setting up a mock context
    // provide a `predecessor` here, it'll modify the default context
    fn get_context(predecessor: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder
    }
    fn setup_contract() -> (VMContextBuilder, Contract) {
        let mut context = VMContextBuilder::new();
        testing_env!(context.predecessor_account_id(accounts(0)).build());
        let contract = Contract::new(
            accounts(0),
            accounts(1),
            accounts(2),
            U128::from(100),
            1000000,
            1000000000,
        );
        (context, contract)
    }

    #[test]
    fn test_new() {
        let (mut context, mut contract) = setup_contract();
        testing_env!(context
            .predecessor_account_id(accounts(0))
            .attached_deposit(1)
            .build());

        assert_eq!(contract.get_owner(), accounts(0));
        contract.update_owner(accounts(1));
        assert_eq!(contract.get_owner(), accounts(1));
    }
    #[test]
    fn test_enable() {
        let (mut context, mut contract) = setup_contract();
        testing_env!(context
            .predecessor_account_id(accounts(0))
            .attached_deposit(1)
            .build());
        assert_eq!(contract.get_enable_status(), false);
        contract.update_enable(true);
        assert_eq!(contract.get_enable_status(), true);
    }
}
