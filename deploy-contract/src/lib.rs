use near_sdk::{env, AccountId, Gas};

#[no_mangle]
pub unsafe fn deploy_contract() {
    let account_id = AccountId::new_unchecked("a.callrs.test.near".to_string());
    let promise_idx = env::promise_batch_create(&account_id);
    env::promise_batch_action_create_account(promise_idx);
    env::promise_batch_action_transfer(promise_idx, 10000000000000000000000000u128);
    let code: &[u8] = include_bytes!("../res/promise_api.wasm");
    env::promise_batch_action_deploy_contract(promise_idx, code);
    env::promise_batch_action_function_call(promise_idx, "cross_contract_callee", b"abc", 0, Gas(20_000_000_000_000u64));
    env::promise_return(promise_idx);
}
