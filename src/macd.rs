use chrono::DateTime;
use json::JsonValue;
use std::error::Error;
use yata::core::Candle;
use yata::core::IndicatorResult;
use yata::indicators::MACD;
use yata::prelude::*;

/*
Get main data to plot
*/
fn sanitize_data(
    stock: JsonValue,
    prices: Vec<f64>,
    macd: f64,
    signal: f64,
) -> Result<JsonValue, Box<dyn Error>> {
    let full_name = stock["chart"]["result"][0]["meta"]["longName"]
        .as_str()
        .unwrap_or("Not found");
    let symbol = stock["chart"]["result"][0]["meta"]["symbol"]
        .as_str()
        .unwrap_or("Not found");
    let timestamps: Vec<_> = stock["chart"]["result"][0]["timestamp"]
        .members()
        .filter_map(|x| x.as_i64())
        .collect();

    let first_time =
        DateTime::from_timestamp(timestamps.first().unwrap_or(&0).clone(), 0).unwrap_or_default();
    let last_time =
        DateTime::from_timestamp(timestamps.last().unwrap_or(&0).clone(), 0).unwrap_or_default();

    let mut result = JsonValue::new_object();
    result["full_name"] = full_name.into();
    result["symbol"] = symbol.into();
    result["first_time"] = first_time.to_rfc3339().into();
    result["last_time"] = last_time.to_rfc3339().into();
    result["last_price"] = (*prices.last().unwrap_or(&0.0_f64)).into();
    result["macd"] = macd.into();
    result["signal"] = signal.into();

    Ok(result)
}

/*
Calculate MACD for all stocks
*/
pub fn calculate_macd(historical: Vec<JsonValue>) -> Result<Vec<JsonValue>, Box<dyn Error>> {
    let mut interesting_stocks: Vec<JsonValue> = vec![];
    for stock in historical {
        let mut prices: Vec<f64> = stock["chart"]["result"][0]["indicators"]["adjclose"][0]
            ["adjclose"]
            .members()
            .filter_map(|x| x.as_f64())
            .collect();

        if prices.len() == 0 {
            println!(
                "No historical data for {}",
                stock["chart"]["result"][0]["meta"]["symbol"]
            );
            continue;
        }
        let first_candle: Candle = Candle {
            open: 0.0_f64,
            high: 0.0_f64,
            low: 0.0_f64,
            volume: 0.0_f64,
            close: *prices.first().unwrap_or(&0.0_f64),
        };
        prices.remove(0);
        let mut macd = MACD::default().init(&first_candle).unwrap();

        let mut result: Option<IndicatorResult> = None;
        for close in &prices {
            let candle = Candle {
                open: 0.0_f64,
                high: 0.0_f64,
                low: 0.0_f64,
                volume: 0.0_f64,
                close: close.to_owned(),
            };
            result = Some(macd.next(&candle));
        }

        if result.is_some() {
            let macd = result.unwrap().values()[0];
            let signal = result.unwrap().values()[1];

            if signal >= macd && (macd < 0.0 || signal < 0.0) {
                interesting_stocks.push(sanitize_data(stock, prices, macd, signal)?);
            }
        }
    }

    Ok(interesting_stocks)
}
