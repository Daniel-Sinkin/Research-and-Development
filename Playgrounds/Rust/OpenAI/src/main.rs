use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use reqwest::Client;
use serde_json::json;
use std::fs;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read API key from file
    let api_key = fs::read_to_string("secret")?.trim().to_string(); // Ensure the file path is correct

    // Set headers
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", api_key))?,
    );
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    // The Client
    let client = Client::builder().default_headers(headers).build()?;

    // Set the URL
    let url = "https://api.openai.com/v1/chat/completions";

    // Set the model and messages
    let body = json!({
        "model": "gpt-3.5-turbo",
        "messages": [
            { "role": "user", "content": "Hello!" }
        ],
        "max_tokens": 250,
    });

    // Send POST request
    let res = client.post(url).json(&body).send().await?;

    // Get the text
    let json: serde_json::Value = res.json().await?;

    // Extract the returned text
    //if let Some(text) = json.get("choices").and_then(|choices| {
    //    choices[0]
    //        .get("message")
    //        .and_then(|message| message.get("content"))
    //}) {
    //    println!("{}", text);
    //}
    if let Some(message) = json
        .get("choices")
        .and_then(|choices| choices[0].get("message"))
    {
        let _role = message.get("role").unwrap().as_str().unwrap();
        let content = message.get("content").unwrap().as_str().unwrap();
        println!("{}", content);
    } else {
        println!("Failed to extract the json");
    }

    Ok(())
}
