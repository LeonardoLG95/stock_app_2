use crate::utils::USER_AGENT;
use futures::future::join_all;
use reqwest::Client;
use scraper::{Html, Selector};
use std::collections::HashSet;
use std::error::Error;
use tokio;

const URL: &str = "https://www.slickcharts.com/{}";
const INDEXES: [&str; 3] = ["sp500", "nasdaq100", "dowjones"];

/**
 * Fetch asynchronously 1 URL and parses it
 */
async fn fetch_url(
    client: Client,
    url: String,
) -> Result<HashSet<String>, Box<dyn Error + Send + Sync>> {
    let body = client
        .get(url)
        .header("User-Agent", USER_AGENT)
        .send()
        .await?
        .text()
        .await?;

    let symbols = get_symbols(body.as_str());
    Ok(symbols?)
}

/**
 * Parses the content from the website to extract the list of symbols
 */
fn get_symbols(data: &str) -> Result<HashSet<String>, Box<dyn Error + Send + Sync>> {
    let document = Html::parse_document(&data);
    let tbody_selector = Selector::parse("tbody").unwrap();

    let mut constituents_table = String::new();
    for element in document.select(&tbody_selector) {
        let html = element.html();
        if html.contains("symbol") {
            constituents_table = html;
            break;
        }
    }

    let pattern = regex::Regex::new(r"/symbol/([A-Za-z0-9]+)")?;
    let mut symbols: HashSet<String> = HashSet::new();
    for cap in pattern.captures_iter(&constituents_table) {
        if let Some(matched) = cap.get(1) {
            symbols.insert(matched.as_str().to_string());
        }
    }

    Ok(symbols)
}

/**
 * Joins everything together to extract all symbols we are interested
 */
pub async fn extract_data() -> Result<HashSet<String>, Box<dyn Error + Send + Sync>> {
    let client = Client::new();
    let mut handles = vec![];
    for index in &INDEXES {
        let handle = tokio::spawn(fetch_url(client.clone(), URL.replace("{}", index)));
        handles.push(handle);
    }

    let results = join_all(handles).await;
    let mut symbols: HashSet<String> = HashSet::new();
    for result in results {
        symbols.extend(result??.into_iter());
    }

    Ok(symbols)
}
