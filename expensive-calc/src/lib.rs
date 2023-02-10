use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::near_bindgen;

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
struct ExpensiveCalc {}

#[near_bindgen]
impl ExpensiveCalc {
    pub fn expensive(&mut self, n: i32) -> i32 {
        let mut ret = 0;
        let mut sign = 1;
        for i in 0..n {
            ret += i * sign;
            sign *= -1;
        }
        ret
    }
}