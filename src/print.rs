use json::JsonValue;
use std::error::Error;

/*
Print basic info regarding the stock
*/
pub fn print_stock(stocks: Vec<JsonValue>) -> Result<(), Box<dyn Error>> {
    for stock in stocks {
        println!("-------------------------------------------------");
        println!("{}", stock["full_name"]);
        println!("Symbol: {}", stock["symbol"]);
        println!("First record time: {}", stock["first_time"]);
        println!("Last record time: {}", stock["last_time"]);
        println!("Last record price: {}$", stock["last_price"]);
        println!("MACD: {}", stock["macd"]);
        println!("Signal: {}", stock["signal"]);
        println!("Sector: {}", stock["sector"]);
        println!("Industry: {}", stock["industry"]);
        println!("-------------------------------------------------");
    }

    Ok(())
}
