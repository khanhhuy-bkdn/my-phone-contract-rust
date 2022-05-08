use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen};

near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct MyPhone {
    phone_number: String,
    phone_name: String,
}

#[near_bindgen]
impl MyPhone {
    pub fn get_phone(&self) -> (String, String) {
        (
            self.phone_number.clone(),
            self.phone_name.clone()
        )
    }

    pub fn set_phone_number(&mut self, new_phone_number: String) {
        self.phone_number = new_phone_number;
        let log_message = format!("New phone number {}", self.phone_number);
        env::log(log_message.as_bytes());
    }

    pub fn set_phone_name(&mut self, new_phone_name: String) {
        self.phone_name = new_phone_name;
        let log_message = format!("New phone name {}", self.phone_name);
        env::log(log_message.as_bytes());
    }

    /// Reset to empty.
    pub fn reset(&mut self) {
        self.phone_number = String::new();
        self.phone_name = String::new();
        env::log(b"Reset phone to empty");
    }
}

/*
 * the rest of this file sets up unit tests
 * to run these, the command will be:
 * cargo test --package rust-counter-tutorial -- --nocapture
 */

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};
    
    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "alice.testnet".to_string(),
            signer_account_id: "robert.testnet".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "jane.testnet".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }

    #[test]
    fn set_phone_number() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = MyPhone { phone_number: String::new(), phone_name: String::new() };
        contract.set_phone_number("0123456789".to_string());
        println!("Phone number: {}, Phone name: {}", contract.get_phone().0, contract.get_phone().1);
        assert_eq!(("0123456789".to_string(), String::new()), contract.get_phone());
    }

    #[test]
    fn set_phone_name() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = MyPhone { phone_number: String::new(), phone_name: String::new() };
        contract.set_phone_name("My IPhone".to_string());
        println!("Phone number: {}, Phone name: {}", contract.get_phone().0, contract.get_phone().1);
        assert_eq!((String::new(), "My IPhone".to_string()), contract.get_phone());
    }

    #[test]
    fn set_message_and_reset() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = MyPhone { phone_number: String::new(), phone_name: String::new() };
        contract.set_phone_number("0123455214123".to_string());
        contract.reset();
        println!("Phone number: {}, Phone name: {}", contract.get_phone().0, contract.get_phone().1);
        assert_eq!((String::new(), String::new()), contract.get_phone());
    }
}