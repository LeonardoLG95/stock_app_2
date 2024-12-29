mod historical;
mod info;
mod macd;
mod print;
mod symbols;
mod utils;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let symbols = symbols::extract_data().await?;
    let historic = historical::extract_historical(symbols).await?;
    let historic_macd = macd::calculate_macd(historic);
    let historic_macd_info = info::extract_industry(historic_macd.unwrap()).await?;
    let _ = print::print_stock(historic_macd_info);
    Ok(())
}
