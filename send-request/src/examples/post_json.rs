use reqwest;
use std::collections::HashMap;
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut map = HashMap::new();
    map.insert("lang", "rust");
    map.insert("body", "json");

    let client = reqwest::Client::new();
    let res = client
        .post("http://httpbin.org/post")
        .json(&map)
        .send()
        .await?;

    let json: Value = res.json().await?;
    // Echoed JSON: "rust"
    println!("Echoed JSON: {}", json["json"]["lang"]);
    Ok(())
}