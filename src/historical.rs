use futures::future::join_all;
use json::JsonValue;
use reqwest::Client;
use std::collections::HashSet;
use std::error::Error;

const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/100.0.4896.99 Safari/537.36";

/*
Outputs the URL + each content in symbols list
*/
fn url(symbol: &str) -> String {
    format!(
        r"https://query1.finance.yahoo.com/v8/finance/chart/{}?interval=1mo&period1=0&period2=9999999999",
        symbol
    )
}

/*
Fetch asynchronously 1 URL and parses it
*/
async fn fetch_url(
    client: Client,
    url: String,
) -> Result<Option<JsonValue>, Box<dyn Error + Send + Sync>> {
    let response: JsonValue = match client
        .get(url)
        .header("User-Agent", USER_AGENT)
        .send()
        .await
    {
        Ok(res) => match res.text().await {
            Ok(json) => json::parse(&json).unwrap(),
            Err(err) => {
                println!("Failed to parse JSON: {}", err);
                return Ok(None);
            }
        },
        Err(err) => {
            println!("Request failed: {}", err);
            return Ok(None);
        }
    };
    Ok(Some(response))
}

/*
Takes all the tasks awaits for them and filter unnecessary nested objects
*/
async fn handle_requests(
    handles: Vec<tokio::task::JoinHandle<Result<Option<JsonValue>, Box<dyn Error + Send + Sync>>>>,
) -> Result<Vec<JsonValue>, Box<dyn Error + Sync + Send>> {
    let raw_data = join_all(handles).await;

    // Filter Nones, Result and Option wrappers
    let filtered_data: Vec<JsonValue> = raw_data
        .into_iter()
        .filter_map(|outer| outer.ok()?.ok()?.map(|value| value))
        .collect();

    Ok(filtered_data)
}

/*
Join all together to return a manageable collection to pass to filters
*/
pub async fn extract_historical(
    symbols: HashSet<String>,
) -> Result<Vec<JsonValue>, Box<dyn Error + Sync + Send>> {
    let client = Client::new();
    let mut handles = vec![];
    for symbol in &symbols {
        let handle = tokio::spawn(fetch_url(client.clone(), url(symbol)));
        handles.push(handle);
    }

    let filtered_data = handle_requests(handles).await?;

    Ok(filtered_data)
}
