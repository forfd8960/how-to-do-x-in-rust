use reqwest;
use reqwest::header::{HeaderMap, USER_AGENT};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, "MyRustBot/1.0".parse()?);
    headers.insert("X-Custom-Header", "secret-value".parse()?);

    let client = reqwest::Client::new();
    let res = client
        .get("http://httpbin.org/headers")
        .headers(headers)
        .send()
        .await?;

    let json: serde_json::Value = res.json().await?;
    // Headers echoed: String("MyRustBot/1.0")
    println!("Headers echoed: {:?}", json["headers"]["User-Agent"]);
    Ok(())
}
