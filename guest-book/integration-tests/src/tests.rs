use near_units::parse_near;
use serde_json::json;
use std::{env, fs};
use workspaces::{Account, Contract, AccountId};
pub struct PostedMessage {
    pub premium: bool,
    pub sender: AccountId,
    pub text: String
}
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let wasm_arg: &str = &(env::args().nth(1).unwrap());
    let wasm_filepath = fs::canonicalize(env::current_dir()?.join(wasm_arg))?;

    let worker = workspaces::sandbox().await?;
    let wasm = std::fs::read(wasm_filepath)?;
    let contract = worker.dev_deploy(&wasm).await?;

    // create accounts
    let account = worker.dev_create_account().await?;
    let alice = account
        .create_subaccount("alice")
        .initial_balance(parse_near!("30 N"))
        .transact()
        .await?
        .into_result()?;

    // begin tests
    test_default(&alice, &contract).await?;
    test_add_message(&alice, &contract).await?;
    Ok(())
}

async fn test_default(user: &Account, contract: &Contract) -> anyhow::Result<()> {
    let total: u64 = user
        .call(contract.id(), "total_messages")
        .args_json(json!({}))
        .transact()
        .await?
        .json()?;

    assert_eq!(total, 0);
    println!("      Passed ✅ test_default");
    Ok(())
}

async fn test_add_message(user: &Account, contract: &Contract) -> anyhow::Result<()> {
    user.call(contract.id(), "add_message")
        .args_json(json!({"text": "Msg 1"}))
        .transact()
        .await?
        .into_result()?;

    let total: u64 = user
        .call(contract.id(), "total_messages")
        .args_json(json!({}))
        .transact()
        .await?
        .json()?;

    let msg: PostedMessage = user
        .call(contract.id(), "get_message")
        .args_json(json!({"from_index": None, "limit": None}))
        .transact()
        .await?
        .json()?[0];

    let expect = PostedMessage {premium: false, sender: *user, text: "Msg 1".to_string()};
    assert_eq!(total, 1);
    assert_eq!(msg, expect);
    println!("      Passed ✅ add msg");
    Ok(())
}
