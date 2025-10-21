use reqwest;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut form_data = HashMap::new();
    form_data.insert("title", "Hello");
    form_data.insert("content", "World");

    let client = reqwest::Client::new();
    let res = client
        .post("http://httpbin.org/post")
        .form(&form_data)
        .send()
        .await?;

    // Form data echoed: Object {"content": String("World"), "title": String("Hello")}
    let json: serde_json::Value = res.json().await?;
    println!("Form data echoed: {:?}", json["form"]);
    Ok(())
}