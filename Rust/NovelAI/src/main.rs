use std::error::Error;
use std::fs;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE, AUTHORIZATION};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Read the API key from the secret file and trim whitespace
    let api_key = fs::read_to_string("secret")?
        .trim()
        .to_string();

    // Set up the headers for the request
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", api_key))?);

    // Set up the body for the request
    let body = json!({
        "input": "Input",
        "model": "clio-v1",
        "parameters": {
            "temperature": 1.05,
            "max_length": 40,
            "min_length": 1,
            "top_k": 79,
            "top_p": 0.95,
            "top_a": 0.075,
            "tail_free_sampling": 0.989,
            "repetition_penalty": 1.5,
            "repetition_penalty_range": 8192,
            "repetition_penalty_frequency": 0.03,
            "repetition_penalty_presence": 0.005,
            "bad_words_ids": [[3],[49356],[1431],[31715],[34387],[20765],[30702],[10691],[49333],[1266],[19438],[43145],[26523],[41471],[2936]],
            "generate_until_sentence": true,
            "use_cache": false,
            "use_string": false,
            "return_full_text": false,
            "prefix": "vanilla",
            "num_logprobs": 10,
            "order": [1,3,4,0,2]
        }
    });

    // Make the request
    let client = reqwest::Client::new();
    let res = client.post("https://api.novelai.net/generation")
        .headers(headers)
        .json(&body)
        .send()
        .await?;

    // Print the text of the response
    println!("{}", res.text().await?);

    Ok(())
}

