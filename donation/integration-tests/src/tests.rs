use near_units::parse_near;
use serde_json::json;
use std::{env, fs};
use workspaces::{Account, Contract};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let wasm_arg: &str = &(env::args().nth(1).unwrap());
    let wasm_filepath = fs::canonicalize(env::current_dir()?.join(wasm_arg))?;

    let worker = workspaces::sandbox().await?;
    let wasm = std::fs::read(wasm_filepath)?;

    // create accounts
    let account = worker.dev_create_account().await?;

    let root = account
        .create_subaccount("root")
        .initial_balance(parse_near!("30 N"))
        .transact()
        .await?
        .into_result()?;

    let alice = account
        .create_subaccount("alice")
        .initial_balance(parse_near!("30 N"))
        .transact()
        .await?
        .into_result()?;

    let bob = account
        .create_subaccount("bob")
        .initial_balance(parse_near!("30 N"))
        .transact()
        .await?
        .into_result()?;

    let contract = root.dev_deploy(&wasm).await?;

    // begin tests
    test_init(&root, &contract, &alice).await?;
    Ok(())
}

async fn test_init(user: &Account, contract: &Contract, benefeciary: &Account) -> anyhow::Result<()> {
    let contract: Contract = user
        .call(contract.id(), "init")
        .args_json(json!({"beneficiary": benefeciary.to_string()}))
        .transact()
        .await?
        .json()?;

    assert_eq!(contract.beneficiary, *benefeciary);
    println!("      Passed ✅ gets init contract");
    Ok(())
}
