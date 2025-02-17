use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::env;
use dotenv::dotenv;
use tokio_postgres::{NoTls, Error};

#[derive(Debug, Serialize, Deserialize)]
struct Transaction {
    hash: String,
    from: String,
    to: String,
    value: String,
    gasPrice: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let api_key = env::var("ALCHEMY_API_KEY").expect("API Key not found in .env");
    let client = Client::new();
    let url = format!("https://eth-mainnet.alchemyapi.io/v2/{}", api_key);
    let payload = json!({
        "jsonrpc": "2.0",
        "method": "eth_getBlockByNumber",
        "params": ["latest", true],
        "id": 1
    });
    let response = client.post(&url).json(&payload).send().await?;
    let response_json: serde_json::Value = response.json().await?;

    println!("Latest Block Data: {:#?}", response_json);
    Ok(())
}