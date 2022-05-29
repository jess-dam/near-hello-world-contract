use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen};

near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct HelloWorld {
    name: String
}

#[near_bindgen]
impl HelloWorld {

    pub fn get_message(&self) -> String {
        return format!("Hello {}!", self.name);
    }

    pub fn set_name(&mut self, name: &String) {
        self.name = name.to_string();
        env::log(format!("Name has now been set to {}", self.name).as_bytes());
    }
}

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
    fn can_set_name() {
        // Given
        let context = get_context(vec![], false);
        testing_env!(context);

        let test_name: String = String::from("Alice");
        let mut hello_world = HelloWorld { name: "".to_string() };

        // When
        hello_world.set_name(&test_name);

        // Then
        let expected_message: String = format!("Hello {}!", test_name);
        assert_eq!(expected_message, hello_world.get_message());
    }
}
