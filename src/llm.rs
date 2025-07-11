use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<Message>,
    temperature: f32,
    max_tokens: u32,
}

#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: MessageContent,
}

#[derive(Deserialize)]
struct MessageContent {
    content: String,
}

pub fn generate_pr_body(diff: &str, summary: &str) -> Result<String, String> {
    let api_key = env::var("OPEN_AI_API_KEY").map_err(|_| "OPEN_AI_API_KEY not set".to_string())?;

    let client = reqwest::blocking::Client::new();

    let prompt = format!(
        "You are a developer assistant. Given the following git diff and ticket summary, write a professional pull request body.\n\n\
        ### Ticket Summary:\n{}\n\n### Git Diff:\n{}\n\n\
        Respond with sections: Summary, Changes, and Context.",
        summary,
        &diff[..diff.len().min(3000)] // truncate for safety
    );

    let body = ChatRequest {
        model: "gpt-4.1".to_string(),
        messages: vec![Message {
            role: "user".to_string(),
            content: prompt,
        }],
        temperature: 0.5,
        max_tokens: 750,
    };

    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(api_key)
        .json(&body)
        .send()
        .map_err(|e| format!("HTTP error: {}", e))?;

    let result: ChatResponse = response.json().map_err(|e| format!("Parse error: {}", e))?;

    Ok(result
        .choices
        .first()
        .map_or("No response".to_string(), |c| c.message.content.clone()))
}

pub fn generate_critiques(diff: &str, summary: &str) -> Result<String, String> {
    let api_key = env::var("OPEN_AI_API_KEY").map_err(|_| "OPEN_AI_API_KEY not set".to_string())?;

    let client = reqwest::blocking::Client::new();

    let prompt = format!(
        "You are a code reviewer. Given the following git diff and ticket summary, provide constructive feedback on the changes.\n\n\
        ### Ticket Summary:\n{}\n\n### Git Diff:\n{}\n\n\
        Respond with sections: Critique, Suggestions, and Questions. Please be succinct, if you have time for nitpicks thats great, but try to prioritize. I am utilizing 2000 tokens maximum. Annotate with Critical, Major, Minor, Nitpick and also postive and negative feedback",
        summary,
        &diff[..diff.len().min(7000)] // truncate for safety
    );

    let body = ChatRequest {
        model: "gpt-4.1".to_string(),
        messages: vec![Message {
            role: "user".to_string(),
            content: prompt,
        }],
        temperature: 0.5,
        max_tokens: 1500,
    };

    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(api_key)
        .json(&body)
        .send()
        .map_err(|e| format!("HTTP error: {}", e))?;

    let result: ChatResponse = response.json().map_err(|e| format!("Parse error: {}", e))?;

    Ok(result
        .choices
        .first()
        .map_or("No response".to_string(), |c| c.message.content.clone()))
}
