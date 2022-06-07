use serde_json::json;
use near_units::parse_near;
use workspaces::prelude::*; 
use workspaces::{network::Sandbox, Account, Contract, Worker};

const WASM_FILEPATH: &str = "../../../../out/main.wasm";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let worker = workspaces::sandbox().await?;
    let wasm = std::fs::read(WASM_FILEPATH)?;
    let contract = worker.dev_deploy(&wasm).await?;

    // create accounts
    let owner = worker.root_account();
    let alice = owner
    .create_subaccount(&worker, "alice")
    .initial_balance(parse_near!("30 N"))
    .transact()
    .await?
    .into_result()?;

    // begin tests  
    test_message(&alice, &contract, &worker).await?;
    test_messages(&alice, &owner, &contract, &worker).await?;
    Ok(())
}   

async fn test_message(
    user: &Account,
    contract: &Contract,
    worker: &Worker<Sandbox>,
) -> anyhow::Result<()> {
    user
    .call(&worker, contract.id(), "addMessage")
    .args_json(json!({"text": "aloha"}))?
    .transact()
    .await?;

    let result: serde_json::Value = user
        .call(&worker, contract.id(), "getMessages")
        .args_json(json!({}))?
        .transact()
        .await?
        .json()?;

    let expected = json!(
        [{
            "premium": false,
            "sender": user.id(),
            "text": "aloha",
        }]
    );    

    assert_eq!(result, expected);
    println!("      Passed ✅ send one message and retrieve it");
    Ok(())
}

async fn test_messages(
    alice: &Account,
    user: &Account,
    contract: &Contract,
    worker: &Worker<Sandbox>,
) -> anyhow::Result<()> {
    user
        .call(&worker, contract.id(), "addMessage")
        .args_json(json!({"text": "hola"}))?
        .transact()
        .await?;

    let result: serde_json::Value = user
        .call(&worker, contract.id(), "getMessages")
        .args_json(json!({}))?
        .transact()
        .await?
        .json()?;

    let expected = json!(
        [{
            "premium": false,
            "sender": alice.id(),
            "text": "aloha",
        },
        {
            "premium": false,
            "sender": user.id(),
            "text": "hola",
        }]
    );    

    assert_eq!(result, expected);
    println!("      Passed ✅ send two messages and expect two total");
    Ok(())
}