use reqwest;

/*
Status: 200 OK
Body: {
  "args": {}, 
  "data": "Hello, server!", 
  "files": {}, 
  "form": {}, 
  "headers": {
    "Accept": "*//*", 
    "Content-Length": "14", 
    "Host": "httpbin.org", 
    "X-Amzn-Trace-Id": "Root=1-68f38a7d-3b3a8bdd28e23c095f4ce675"
  }, 
  "json": null, 
  "origin": "xxx", 
  "url": "http://httpbin.org/post"
}
*/

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let res = client
        .post("http://httpbin.org/post")
        .body("Hello, server!")
        .send()
        .await?;

    // Status: 200 OK
    println!("Status: {}", res.status());
    println!("Body: {}", res.text().await?);
    Ok(())
}