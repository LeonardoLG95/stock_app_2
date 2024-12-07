mod symbols;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    match symbols::extract_data().await {
        Ok(tickers) => {
            // Print the contents of the HashSet
            println!("Tickers: {:?}", tickers);
        }
        Err(e) => {
            // Handle the error case
            println!("Error: {}", e);
        }
    }
    Ok(())
}
