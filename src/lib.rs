use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    assert_one_yocto, env, ext_contract, near_bindgen, promise_result_as_success, AccountId,
    Balance, BorshStorageKey, CryptoHash, Gas, PanicOnDefault, Promise,
};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Default)]
pub struct Contract {

}

#[near_bindgen]
impl Contract {
    pub fn log_signer(&self) {
        env::log_str(format!("signer: {}", env::signer_account_id()).as_str());
        env::log_str(format!("predessor: {}", env::predecessor_account_id()).as_str());
    }
}