use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, PanicOnDefault};

#[near_bindgen]
#[derive(PanicOnDefault, BorshDeserialize, BorshSerialize)]
pub struct Counter {
    val: u64,
}

#[near_bindgen]
impl Counter {
    #[init]
    pub fn new() -> Self {
        Self{
            val: 0
        }
    }

    pub fn get_num(&self) -> u64 {
        return self.val;
    }

    pub fn increment(&mut self, value: u64) {
        self.val += value;
    }
}