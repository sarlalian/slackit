use clap::Parser;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde::Serialize;
use std::env; // Not strictly needed if using clap's env feature directly, but good practice

// Define the structure for command-line arguments
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The message text to send to Slack
    #[arg(short, long)]
    message: String,

    /// Slack API Token (can also be set via SLACK_TOKEN env var)
    #[arg(short, long, env = "SLACK_TOKEN")]
    token: Option<String>, // Option allows it to be missing if env var is set

    /// Slack Channel ID or Name to post to (can also be set via SLACK_CHANNEL env var)
    #[arg(short, long, env = "SLACK_CHANNEL")]
    channel: Option<String>, // Option allows it to be missing if env var is set
}

// Structure for the Slack API JSON payload
#[derive(Serialize)]
struct SlackMessagePayload<'a> {
    channel: &'a str,
    text: &'a str,
}

// Use tokio runtime for async operations
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse arguments (including environment variables specified in `arg` attributes)
    let cli = Cli::parse();

    // --- Validation ---
    // Clap handles reading from env vars if the arg isn't provided,
    // but we still need to ensure we *have* a token and channel from somewhere.
    let token = cli.token.ok_or("Error: Slack token is required. Provide --token or set SLACK_TOKEN environment variable.")?;
    let channel = cli.channel.ok_or("Error: Slack channel is required. Provide --channel or set SLACK_CHANNEL environment variable.")?;
    // --- End Validation ---

    println!(
        "Attempting to send message to channel '{}'...",
        channel
    );

    // Create the payload
    let payload = SlackMessagePayload {
        channel: &channel,
        text: &cli.message,
    };

    // Create an HTTP client
    let client = reqwest::Client::new();

    // Slack API endpoint for posting messages
    let api_url = "https://slack.com/api/chat.postMessage";

    // Send the request
    let response = client
        .post(api_url)
        .header(AUTHORIZATION, format!("Bearer {}", token))
        .header(CONTENT_TYPE, "application/json; charset=utf-8")
        .json(&payload) // reqwest handles serializing the payload struct to JSON
        .send()
        .await?;

    // Check the response status and body
    if response.status().is_success() {
        // Optionally, parse the JSON response body to check Slack's 'ok' field
        let response_body: serde_json::Value = response.json().await?;
        // println!("Slack API Response: {}", response_body); // Uncomment for debugging

        if response_body["ok"].as_bool().unwrap_or(false) {
            println!("Message posted successfully!");
            Ok(())
        } else {
            let error_message = response_body["error"].as_str().unwrap_or("Unknown Slack API error");
            eprintln!("Error from Slack API: {}", error_message);
            // Convert the Slack API error into a standard Rust error
            Err(error_message.into())
        }
    } else {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_else(|_| "Could not read error body".to_string());
        eprintln!("Error sending message: HTTP Status {} - {}", status, error_text);
        // Convert the HTTP error into a standard Rust error
        Err(format!("HTTP Error: {} - {}", status, error_text).into())
    }
}
