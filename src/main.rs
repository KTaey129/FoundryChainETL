use reqwest;
use serde_json::Value;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "https://api.blockchain.com/v3/exchange/tickers/BTC-USD";
    let response = reqwest::get(url).await?.text().await?;

    let json: Value = serde_json::from_str(&response)?;
    println!("Blockchain Data: {:?}", json);

    Ok(())
}