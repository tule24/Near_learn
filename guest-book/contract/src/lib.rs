use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::Vector;
use near_sdk::json_types::U128;
use near_sdk::serde::Serialize;
use near_sdk::{log, near_bindgen, AccountId, env, Balance};

const POINT_ONE: Balance = 100_000_000_000_000_000_000_000;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct PostedMessage {
    pub premium: bool,
    pub sender: AccountId,
    pub text: String
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
struct GuestBook {
    messages: Vector<PostedMessage>    
}

impl Default for GuestBook {
    fn default() -> Self {
        Self { messages: Vector::new(b'm') }
    }
}

#[near_bindgen]
impl GuestBook {
    #[payable]
    pub fn add_message(&mut self, text: String) {
        // If the user attaches more than 0.1N the message is premium
        let premium = env::attached_deposit() >= POINT_ONE;
        let sender = env::predecessor_account_id();

        let message = PostedMessage {premium, sender, text};
        self.messages.push(&message);
    }

    pub fn get_message(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<PostedMessage> {
        let from = u128::from(from_index.unwrap_or(U128(0)));

        self.messages.iter()
                    .skip(from as usize)
                    .take(limit.unwrap_or(10) as usize)
                    .collect()
    }

    pub fn total_messages(&self) -> u64 {
        self.messages.len()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_book() {
        let contract = GuestBook::default();
        assert_eq!(
            contract.messages.len(),
            0
        );
    }

    #[test]
    fn add_msg() {
        let mut contract = GuestBook::default();
        contract.add_message("Msg 1".to_string());

        let posted_msg = &contract.get_message(None, None)[0];
        assert_eq!(
            contract.messages.len(),
            1
        );
        assert_eq!(
            posted_msg.premium,
            false
        );
        assert_eq!(
            posted_msg.text,
            "Msg 1".to_string()
        );
    }

    #[test]
    fn iters_msg() {
        let mut contract = GuestBook::default();
        contract.add_message("Msg 1".to_string());
        contract.add_message("Msg 2".to_string());
        contract.add_message("Msg 3".to_string());

        let total_msg = &contract.total_messages();
        assert_eq!(*total_msg, 3);

        let last_msg = &contract.get_message(Some(U128::from(1)), Some(2))[1];

        assert_eq!(
            last_msg.premium,
            false
        );
        assert_eq!(
            last_msg.text,
            "Msg 3".to_string()
        );
    }
}
