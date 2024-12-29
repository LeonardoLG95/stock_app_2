use crate::utils::fetch_url;
use crate::utils::handle_requests;
use json::JsonValue;
use reqwest::Client;
use std::error::Error;

const URL: &str = r"https://api.nasdaq.com/api/quote/{}/summary?assetclass=stocks";

pub async fn extract_industry(
    historical_macd: Vec<JsonValue>,
) -> Result<Vec<JsonValue>, Box<dyn Error + Sync + Send>> {
    println!("Test");
    let client = Client::new();
    let mut handles = vec![];
    for stock in &historical_macd {
        let composed_url = URL.replace("{}", stock["symbol"].as_str().unwrap_or_default());
        println!("{}", composed_url);
        let handle = tokio::spawn(fetch_url(client.clone(), composed_url));
        handles.push(handle);
    }
    let industries = handle_requests(handles).await?;

    let mut histocial_macd_industry = vec![];
    for stock_industry in industries {
        for stock_data in &historical_macd {
            if stock_data["symbol"] == stock_industry["data"]["symbol"] {
                let mut stock_data_owner = stock_data.to_owned();
                stock_data_owner["sector"] = stock_industry["data"]["summaryData"]["Sector"]
                    ["value"]
                    .as_str()
                    .into();
                stock_data_owner["industry"] = stock_industry["data"]["summaryData"]["Industry"]
                    ["value"]
                    .as_str()
                    .into();
                histocial_macd_industry.push(stock_data_owner);
            }
        }
    }

    Ok(histocial_macd_industry)
}
