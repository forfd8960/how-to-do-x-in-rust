use reqwest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let body = reqwest::get("https://www.rust-lang.org")
        .await?
        .text()
        .await?;
    println!("Body: {}", &body[..300]);
    Ok(())
}
/*
Body: <!doctype html>
<html lang="en-US">
  <head>
    <meta charset="utf-8">
    <title>
            Rust Programming Language
        </title>
    <meta name="viewport" content="width=device-width,initial-scale=1.0">
    <meta name="description" content="A language empowering everyone to build reliable 
*/