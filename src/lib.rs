use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{
     env, ext_contract, near_bindgen, promise_result_as_success, AccountId,
    Balance, CryptoHash, Gas, PanicOnDefault, Promise, require,
};

//const FACTORY_ACCOUNT_STR: &str = "contract_one.jeph.testnet";

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    pub owner_id: AccountId,
    pub user_id: AccountId,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(user_id: AccountId) -> Self {
        let owner_id: AccountId = env::predecessor_account_id();
        Self { 
            owner_id, 
            user_id 
        }
    }

    pub fn delete_contract(&mut self) {
        let mut correct_caller: bool = false;
        if env::signer_account_id() == self.user_id || env::predecessor_account_id() == self.owner_id {
            correct_caller = true;
        }
        require!(correct_caller, "Only the owner or the user can delete the contract");
        Promise::new(AccountId::from(env::current_account_id())).delete_account(AccountId::from(self.owner_id.clone()));
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
        ).then(
            ext_self::autodestruction(
                self.user_id.clone(),
                amount,
                env::current_account_id(),
                 0,
                  Gas(7_000_000_000_000)),
        ).then(
            //delete_contract(self.owner_id.clone()),
            Promise::new(env::current_account_id())
        );
        //TODO:
        //Si fue exitoso llamar a la función delete del contrato principal.
    }

    pub fn autodestruction(&mut self, merchant_id: AccountId, amount: Balance) {
        env::log_str("autodestruction");
        env::log_str(format!("signer: {}", env::signer_account_id()).as_str());
        env::log_str(format!("predecessor: {}", env::predecessor_account_id()).as_str());
        env::log_str(format!("owner: {}", self.owner_id).as_str());
        env::log_str(format!("user: {}", self.user_id).as_str());
        env::log_str(format!("merchant: {}", merchant_id).as_str());
        env::log_str(format!("amount: {}", amount).as_str());
        env::log_str(format!("promise_result_as_success: {:?}", promise_result_as_success()).as_str());
        env::log_str(format!("promise_result_as_success: {:#?}", promise_result_as_success()).as_str());
        env::log_str(format!("attached_gas: {:?}", env::prepaid_gas()).as_str());
        env::log_str(format!("attached_gas: {:#?}", env::prepaid_gas()).as_str());
        env::log_str(format!("used_gas: {:?}", env::used_gas()).as_str());
        env::log_str(format!("used_gas: {:#?}", env::used_gas()).as_str());
        env::log_str(format!("result: {:?}", env::promise_result(0)).as_str());
        env::log_str(format!("result: {:#?}", env::promise_result(0)).as_str());
        require!(promise_result_as_success() != None, "No se pudo transferir el dinero, no hay suficiente");

        self.delete_contract();

            ext_transfer::add_balance_to_merchant(
            merchant_id, 
            env::current_account_id(), 
            amount, 
            self.owner_id.clone(), 
            0, 
            Gas(5_000_000_000_000),
        );

    }

}

#[ext_contract(ext_transfer)]
pub trait ExtTransfer {
    fn ft_transfer(&self, receiver_id: String, amount: String, memo: String);
    fn add_balance_to_merchant(&mut self, merchant_id: AccountId, sub_id: AccountId, ammount: u128);
}

#[ext_contract(ext_self)]
pub trait ExtSelf {
    fn autodestruction(&self, merchant_id: AccountId, amount: Balance);
    fn delete_contract(&mut self);
}