use crate::utils::fetch_url;
use crate::utils::handle_requests;
use json::JsonValue;
use reqwest::Client;
use std::collections::HashSet;
use std::error::Error;

const URL: &str = r"https://query1.finance.yahoo.com/v8/finance/chart/{}?interval=1mo&period1=0&period2=9999999999";

/*
Join all together to return a manageable collection to pass to filters
*/
pub async fn extract_historical(
    symbols: HashSet<String>,
) -> Result<Vec<JsonValue>, Box<dyn Error + Sync + Send>> {
    let client = Client::new();
    let mut handles = vec![];
    for symbol in &symbols {
        let handle = tokio::spawn(fetch_url(client.clone(), URL.replace("{}", symbol)));
        handles.push(handle);
    }

    let filtered_data = handle_requests(handles).await?;

    Ok(filtered_data)
}
