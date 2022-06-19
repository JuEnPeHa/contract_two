use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    assert_one_yocto, env, ext_contract, near_bindgen, promise_result_as_success, AccountId,
    Balance, BorshStorageKey, CryptoHash, Gas, PanicOnDefault, Promise,
};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    pub owner_id: AccountId,
    pub user_id: AccountId,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(owner_id: AccountId, user_id: AccountId) -> Self {
        Self { owner_id, user_id }
    }

    pub fn transfer_usdc(&self, amount: Balance) {
        let account_id = &self.owner_id;
        ext_transfer::ft_transfer(
            account_id.to_string(),
            amount.to_string(),
            "".to_string(),
            AccountId::new_unchecked("usdc.fakes.testnet".to_string()),
            1,
            Gas(5_000_000_000_000),
        ).then(|_| {
            promise_result_as_success(())
        });
        //TODO:
        //Si fue exitoso llamar a la función delete del contrato principal.
    }

    pub fn autodestruction(&self, merchant_id: AccountId, amount: Balance) {
        env::log_str("autodestruction");
        env::log_str(format!("signer: {}", env::signer_account_id()).as_str());
        env::log_str(format!("predessor: {}", env::predecessor_account_id()).as_str());
        env::log_str(format!("owner: {}", self.owner_id).as_str());
        env::log_str(format!("user: {}", self.user_id).as_str());
        ext_transfer::destroy_sub_account(
            merchant_id, 
            sub_id, 
            ammount, 
            account_id, 
            balance, 
           gas
        );
    }

    pub fn log_signer(&self) {
        env::log_str(format!("signer: {}", env::signer_account_id()).as_str());
        env::log_str(format!("predessor: {}", env::predecessor_account_id()).as_str());
    }


}

#[ext_contract(ext_transfer)]
pub trait ExtExample {
    fn ft_transfer(&self, receiver_id: String, amount: String, memo: String);
    fn destroy_sub_account(&mut self, merchant_id: AccountId, sub_id: AccountId, ammount: u128);
}

#[ext_contract(ext_self)]
pub trait ExtSelf {
    fn autodestruction(&self);
    fn log_signer(&self);
}