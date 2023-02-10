use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen};
use near_sdk::collections::UnorderedMap;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
struct HighlevelCollection {
    unorderedMap: UnorderedMap<String, String>,
}

impl Default for HighlevelCollection {
    fn default() -> Self {
        Self { unorderedMap: UnorderedMap::new(b"a") }
    }
}

#[near_bindgen]
impl HighlevelCollection {
    pub fn set(&mut self, key: String, value: String) {
        self.unorderedMap.insert(&key, &value);
    }
}
