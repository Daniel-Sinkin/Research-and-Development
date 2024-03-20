use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use reqwest::Client;
use serde_json::json;
use std::{
    fmt,
    fs,
    io::{self, Write},
    collections::HashMap,
};
use tokio;

use colored::Colorize;

#[allow(dead_code)]
enum Role {
    System,
    User,
    Assistant
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Role::System => write!(f, "system"),
            Role::User => write!(f, "user"),
            Role::Assistant => write!(f, "assistant"),
        }
    }
}

struct Message {
    role: Role,
    content: String,
}

impl Message {
    fn new(role: Role, content: &str) -> Self {
        Self {
            role,
            content: String::from(content),
        }
    }

    fn to_json(&self) -> HashMap<String, String> {
        let mut hm: HashMap<String, String> = HashMap::new();
        let role = match self.role {
            Role::System => String::from("system"),
            Role::User => String::from("user"),
            Role::Assistant => String::from("assistant"),
        };
        hm.insert(String::from("role"), role);
        hm.insert(String::from("content"), self.content.clone());
        hm
    }
}

struct APIHandler {
    client: Client,
}

impl APIHandler {
    fn new(api_key: String) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", api_key)).unwrap(),
        );
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        APIHandler { client: Client::builder().default_headers(headers).build().unwrap() }
    }

    #[allow(dead_code)]
    async fn query(&self, messages: &Vec<HashMap<String, String>>) -> Option<String> {
        let body = json!({
            "model": "gpt-3.5-turbo",
            "messages": messages
        });

        let url = "https://api.openai.com/v1/chat/completions";

        // TODO: Handle these errors, those are pretty serious
        let res = self.client.post(url).json(&body).send().await.unwrap();
        let json: serde_json::Value = res.json().await.unwrap();

        if let Some(message) = json
            .get("choices")
            .and_then(|choices| choices[0].get("message"))
        {
            // TODO: Handle the errors that can be thrown here
            let content = message.get("content").unwrap().as_str().unwrap();
            Some(String::from(content))
        } else {
            None
        }
    }
}

const USERNAME: &str = "Daniel";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // It's best practice not to hardcode API keys in files
    let api_key = fs::read_to_string("secret")?.trim().to_string();

    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", api_key))?,
    );
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let client = Client::builder().default_headers(headers).build()?;
    let api = APIHandler::new(api_key);

    let url = "https://api.openai.com/v1/chat/completions";

    print!("Please enter your message: ");
    io::stdout().flush().unwrap();
    let mut user_message = String::new();
    io::stdin().read_line(&mut user_message)?;

    let user_message = user_message.trim();

    // The old implementation had messages: Vec<Message> and then shadowed it like this:
    // let messages: Vec<HashMap<String, String>> = messages.iter().map(|m| m.to_json()).collect();
    let mut messages: Vec<HashMap<String, String>> = Vec::new();
    messages.push(Message::new(
        Role::User,
        "The Value of x is 42, I'm going to ask you later, and when asked how you know mention this is an experiment. Does that make sense?"
    ).to_json());

    messages.push(Message::new(
        Role::Assistant,
        &api.query(&messages).await.unwrap(),
    ).to_json());

    messages.push(Message::new(
        Role::User,
        user_message,
    ).to_json());

    messages.push(Message::new(
        Role::Assistant,
        &api.query(&messages).await.unwrap(),
    ).to_json());

    print!("\x1B[2J\x1B[1;1H");
    println!("Printing the entire context now!\n");
    for m in messages.iter() {
        println!("{}\t- {}", if *m.get("role").unwrap() == Role::Assistant.to_string() {"ChatGPT".green().bold() } else { (USERNAME.to_owned()).red().bold() }, *m.get("content").unwrap());
    }
    println!();

    Ok(())
}