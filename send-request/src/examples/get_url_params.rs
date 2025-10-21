use reqwest;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut params = HashMap::new();
    params.insert("lang", "rust");
    params.insert("version", "1.75");

    let client = reqwest::Client::new();
    let res = client
        .get("http://httpbin.org/get")
        .query(&params)
        .send()
        .await?;

    // Query params echoed: Object {"lang": String("rust"), "version": String("1.75")}
    let json: serde_json::Value = res.json().await?;
    println!("Query params echoed: {:?}", json["args"]);
    Ok(())
}