use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::near_bindgen;

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
struct HighlevelMinimal {}

#[near_bindgen]
impl HighlevelMinimal {
    pub fn empty(&mut self) {}
}