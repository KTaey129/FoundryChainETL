use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::env;
use dotenv::dotenv;
use tokio_postgres::{NoTls, Error};
use rust_decimal::Decimal;


#[derive(Debug, Serialize, Deserialize)]
struct Transaction {
    hash: String,
    from: String,
    to: String,
    value: Decimal,
    gas_price: Decimal,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let api_key = env::var("ALCHEMY_API_KEY")?;
    let db_url = env::var("DATABASE_URL")?;

    // Connect to PostgreSQL
    let (client, connection) = tokio_postgres::connect(&db_url, NoTls).await?;
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    // Fetch Ethereum block data
    let client_http = Client::new();
    let url = format!("https://eth-mainnet.alchemyapi.io/v2/{}", api_key);
    let payload = json!({
        "jsonrpc": "2.0",
        "method": "eth_getBlockByNumber",
        "params": ["latest", true],
        "id": 1
    });

    let response = client_http.post(&url).json(&payload).send().await?;
    let response_json: serde_json::Value = response.json().await?;

    if let Some(transactions) = response_json["result"]["transactions"].as_array() {
        for tx in transactions {
            let tx_data = Transaction {
                hash: tx["hash"].as_str().unwrap_or_default().to_string(),
                from: tx["from"].as_str().unwrap_or_default().to_string(),
                to: tx["to"].as_str().unwrap_or_default().to_string(),
                value: tx["value"].as_str().unwrap_or_default().parse::<i64>().unwrap_or_default().into(),
                gas_price: tx["gasPrice"].as_str().unwrap_or_default().parse::<i64>().unwrap_or_default().into(),
            };

            // Save to PostgreSQL
            client.execute(
                "INSERT INTO transactions (tx_hash, from_address, to_address, value, gas_price)
                VALUES ($1, $2, $3, $4, $5)
                ON CONFLICT (tx_hash) DO NOTHING",
                &[
                    &tx_data.hash, 
                    &tx_data.from, 
                    &tx_data.to, 
                    &tx_data.value, 
                    &tx_data.gas_price,
                ],
            ).await?;
        }
    }

    println!("Completed saving transaction!");
    Ok(())
}