#![cfg(test)]

use serde_json::json;
use workspaces::prelude::*;
use workspaces::{Worker, DevNetwork, Contract};

const COUNTER_WASM_FILEPATH: &str = "../res/non_fungible_token.wasm";
const APPROVAL_WASM_FILEPATH: &str = "../res/approval_receiver.wasm";
const TOKEN_WASM_FILEPATH: &str = "../res/token_receiver.wasm";

async fn setup() -> anyhow::Result<(workspaces::Worker<impl DevNetwork>, workspaces::Contract)> {
    let worker = workspaces::sandbox();
    let wasm = std::fs::read(COUNTER_WASM_FILEPATH)?;
    let contract = worker.dev_deploy(wasm).await?;
    Ok((worker, contract))
}

#[tokio::test]
async fn nft() -> anyhow::Result<()> {
    let (worker, contract) = setup().await?;

    Ok(())
}