use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{log, near_bindgen};

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Counter {
    val: i8
}

#[near_bindgen]
impl Counter {
    pub fn get_num(&self) -> i8 {
        self.val
    }

    pub fn increment(&mut self) {
        self.val += 1;
        log!("Increased number to {}", self.val);
    }

    pub fn decrement(&mut self) {
        self.val -= 1;
        log!("Decreased number to {}", self.val);
    }

    pub fn reset(&mut self) {
        self.val = 0;
        log!("Reset counter to zero")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_default() {
        let contract = Counter::default();
        let val = contract.get_num();
        assert_eq!(val, 0)
    }

    #[test]
    fn check_increment() {
        let mut contract = Counter::default();
        contract.increment();
        let val = contract.get_num();
        assert_eq!(val, 1)
    }

    #[test]
    fn check_decrement() {
        let mut contract = Counter::default();
        contract.decrement();
        let val = contract.get_num();
        assert_eq!(val, -1)
    }

    #[test]
    #[should_panic]
    fn panics_on_overflow() {
        let mut contract = Counter { val: 127};
        contract.increment();
    }

    #[test]
    #[should_panic]
    fn panics_on_underflow() {
        let mut contract = Counter { val: -128};
        contract.decrement();
    }
}