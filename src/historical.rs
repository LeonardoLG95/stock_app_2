use futures::future::join_all;
use reqwest::Client;
use serde_json::Value;
use std::collections::HashSet;
use std::error::Error;

const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/100.0.4896.99 Safari/537.36";

/**
 * Outputs the URL + each content in symbols list
 */
fn url(symbol: &str) -> String {
    format!(
        r"https://query1.finance.yahoo.com/v8/finance/chart/{}?interval=1mo&period1=0&period2=9999999999",
        symbol
    )
}

/**
 * Fetch asynchronously 1 URL and parses it
 */
async fn fetch_url(
    client: Client,
    url: String,
) -> Result<Option<Value>, Box<dyn Error + Send + Sync>> {
    let response: Value = match client
        .get(url)
        .header("User-Agent", USER_AGENT)
        .send()
        .await
    {
        Ok(res) => match res.json().await {
            Ok(json) => json,
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

/**
 * Join all together to return a manageable collection to pass to filters
 */
pub async fn extract_historical(
    symbols: HashSet<String>,
) -> Result<(), Box<dyn Error + Sync + Send>> {
    let client = Client::new();
    let mut handles = vec![];
    for symbol in &symbols {
        let handle = tokio::spawn(fetch_url(client.clone(), url(symbol)));
        handles.push(handle);
    }

    let results = join_all(handles).await;

    // Filter Nones
    let values: Vec<_> = results.into_iter().filter_map(|x| Some(x)).collect();

    let total = values.len();
    // for value in values {
    //     println!("{:?}", value);
    // }
    println!("Total stocks pulled: {}", total);

    Ok(())
}
