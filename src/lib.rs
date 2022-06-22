use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{
     env, ext_contract, near_bindgen, promise_result_as_success, AccountId,
    Balance, Gas, PanicOnDefault, Promise, require,
};

//const FACTORY_ACCOUNT_STR: &str = "contract_one.jeph.testnet";

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    pub owner_id: AccountId,
    pub user_id: AccountId,
    pub mediator_id: AccountId,
    pub required_amount: Balance,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(user_id: AccountId, required_amount: Balance) -> Self {
        let owner_id: AccountId = env::predecessor_account_id();
        let mediator_id: AccountId = AccountId::new_unchecked("jeph.testnet".to_string());
        Self { 
            owner_id, 
            user_id,
            mediator_id,
            required_amount,
        }
    }

    pub fn update_new(&mut self, /*new_user_id: AccountId,*/ new_required_amount: Balance) {
        //self.user_id = new_user_id;
        self.required_amount = new_required_amount;
        // Self { 
        //     owner_id: self.owner_id, 
        //     user_id: self.user_id, 
        //     mediator_id: self.mediator_id, 
        //     required_amount: new_required_amount,
        // }
    }

    pub fn delete_contract(&mut self) {
        if env::predecessor_account_id() == self.owner_id {
            //require!(env::promise_results_count() == 0, "There are pending promises");
            require!(promise_result_as_success() != None, "No se pudo transferir el dinero, no hay suficiente");
        }
        let mut correct_caller: bool = false;
        if env::signer_account_id() == self.user_id 
        || env::predecessor_account_id() == env::current_account_id()
        || env::signer_account_id() == self.mediator_id {
            correct_caller = true;
        }
        require!(correct_caller, "Only the owner or the user can delete the contract");
        Promise::new(AccountId::from(env::current_account_id())).delete_account(AccountId::from(self.owner_id.clone()));
    }

    pub fn transfer_usdc(&self/*, amount: Balance*/) {
        let account_id = &self.owner_id;
        ext_transfer::ft_transfer(
            account_id.to_string().clone(),
            self.required_amount.to_string(),
            "".to_string(),
            AccountId::new_unchecked("usdc.fakes.testnet".to_string()),
            1,
            Gas(5_000_000_000_000),
        ).then(
            ext_transfer::add_balance_to_merchant(
                self.user_id.clone(), 
                env::current_account_id(), 
                self.required_amount.clone(), 
                self.owner_id.clone(), 
                0, 
                Gas(5_000_000_000_000),
            )
        );
    }

}

#[ext_contract(ext_transfer)]
pub trait ExtTransfer {
    fn ft_transfer(&self, receiver_id: String, amount: String, memo: String);
    fn add_balance_to_merchant(&mut self, merchant_id: AccountId, sub_id: AccountId, amount: u128);
}

#[ext_contract(ext_self)]
pub trait ExtSelf {
    fn delete_contract(&mut self);
}