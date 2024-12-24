mod historical;
mod macd;
mod symbols;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let symbols = symbols::extract_data().await?;
    let historic = historical::extract_historical(symbols).await?;
    let _historic_macd = macd::calculate_macd(historic);
    // println!("{}", historic);
    Ok(())
}
