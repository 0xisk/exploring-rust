use anyhow::{anyhow, Context, Ok, Result};
use mini_redis::client;

/// A simple binary executable program for testing mini-redis
///     1. Connect to mini-redis server
///     2. Set an endpoint to get a message
///     3. Get a response
#[tokio::main]
async fn main() -> Result<()> {
    // Open a connection to mini-redis server
    let mut client = client::connect("localhost:3001")
        .await
        .map_err(|e| anyhow!("Failed to connect to the mini-redis server: {}", e))?;

    // Set an endpoint
    client
        .set("hello", "beautiful world!".into())
        .await
        .map_err(|e| anyhow!("Failed to set an endpoint: {}", e))?;

    // Get the value when you call the endpoint
    let result = client
        .get("hello")
        .await
        .map_err(|e| anyhow!("Failed to get a response: {}", e))?
        .ok_or_else(|| anyhow!("No result found!"))?;

    // Print the result
    println!(
        "Successfully getting the result form the server: {:?}",
        result
    );

    Ok(())
}
