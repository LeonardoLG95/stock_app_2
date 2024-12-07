mod symbols;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    match symbols::extract_data().await {
        Ok(tickers) => {
            println!("Tickers: {:?}", tickers);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
    Ok(())
}
