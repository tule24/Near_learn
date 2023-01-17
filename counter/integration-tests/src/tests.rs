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
    test_default_num(&alice, &contract).await?;
    test_increse_num(&alice, &contract).await?;
    test_decrease_num(&alice, &contract).await?;
    test_reset_num(&alice, &contract).await?;

    Ok(())
}

async fn test_default_num(user: &Account, contract: &Contract) -> anyhow::Result<()> {
    let num: i8 = user
        .call(contract.id(), "get_num")
        .args_json(json!({}))
        .transact()
        .await?
        .json()?;

    assert_eq!(num, 0);
    println!("      Passed ✅ gets default num");
    Ok(())
}

async fn test_increse_num(user: &Account, contract: &Contract) -> anyhow::Result<()> {
    user.call(contract.id(), "increment")
        .args_json(json!({}))
        .transact()
        .await?
        .into_result()?;

    let num: i8 = user
        .call(contract.id(), "get_num")
        .args_json(json!({}))
        .transact()
        .await?
        .json()?;

    assert_eq!(num, 1);
    println!("      Passed ✅ increase num");
    Ok(())
}

async fn test_decrease_num(user: &Account, contract: &Contract) -> anyhow::Result<()> {
    user.call(contract.id(), "reset")
        .args_json(json!({}))
        .transact()
        .await?
        .into_result()?;
    
    user.call(contract.id(), "decrement")
        .args_json(json!({}))
        .transact()
        .await?
        .into_result()?;

    let num: i8 = user
        .call(contract.id(), "get_num")
        .args_json(json!({}))
        .transact()
        .await?
        .json()?;

    assert_eq!(num, -1);
    println!("      Passed ✅ decrease num");
    Ok(())
}

async fn test_reset_num(user: &Account, contract: &Contract) -> anyhow::Result<()> {
    user.call(contract.id(), "reset")
        .args_json(json!({}))
        .transact()
        .await?
        .into_result()?;

    let num: i8 = user
        .call(contract.id(), "get_num")
        .args_json(json!({}))
        .transact()
        .await?
        .json()?;

    assert_eq!(num, 0);
    println!("      Passed ✅ reset num");
    Ok(())
}
