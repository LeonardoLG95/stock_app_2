mod historical;
mod symbols;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    match symbols::extract_data().await {
        Ok(symbols) => {
            historical::extract_historical(symbols).await?;
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
    Ok(())
}

// test