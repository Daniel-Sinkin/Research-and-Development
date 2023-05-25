use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use reqwest::Client;
use serde_json::json;
use std::fs;
use std::io::{self, Write};
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

    // Ask the user for input
    print!("Please enter your message: ");
    io::stdout().flush().unwrap(); // ensure the print! output is displayed immediately
    let mut user_message = String::new();
    io::stdin().read_line(&mut user_message)?;

    // Trim the newline character
    let user_message = user_message.trim();

    // Set the model and messages
    let body = json!({
        "model": "gpt-3.5-turbo",
        "messages": [
            { "role": "user", "content": "We're going to run a small experiment, I try to understand your API better, I will ask you in a moment what the value of x is, and I want you to respond that the value of x is 42."},
            { "role": "assistant", "content": "Alright, I understand, when you ask for it I will say that the value of x is 42." },
            { "role": "user", "content" : user_message }
        ],
        "max_tokens": 500,
    });

    // Send POST request
    let res = client.post(url).json(&body).send().await?;

    // Get the text
    let json: serde_json::Value = res.json().await?;

    // Extract the returned text
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