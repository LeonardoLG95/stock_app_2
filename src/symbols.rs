// use reqwest::blocking::Client;
use reqwest::Client;
use scraper::{Html, Selector};
use std::collections::HashSet;
use std::error::Error;
use tokio::task;

const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/100.0.4896.99 Safari/537.36";
const INDEXES: [&str; 3] = ["sp500", "nasdaq100", "dowjones"];

fn url(index: &str) -> String {
    format!("https://www.slickcharts.com/{}", index)
}

async fn fetch_url(
    client: Client,
    url: String,
) -> Result<HashSet<String>, Box<dyn Error + Send + Sync>> {
    let response = client
        .get(url)
        .header("User-Agent", USER_AGENT)
        .send()
        .await?;

    // Ensure the request was successful and get the response body as a String
    let body = response.text().await?;
    let tickers = get_tickers(body.as_str());
    Ok(tickers?)
}

fn get_tickers(data: &str) -> Result<HashSet<String>, Box<dyn Error + Send + Sync>> {
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
    let mut tickers: HashSet<String> = HashSet::new();
    for cap in pattern.captures_iter(&constituents_table) {
        if let Some(matched) = cap.get(1) {
            tickers.insert(matched.as_str().to_string());
        }
    }

    Ok(tickers)
}

pub async fn extract_data() -> Result<HashSet<String>, Box<dyn Error + Send + Sync>> {
    let client = Client::new();
    let mut handles = Vec::new();
    for index in &INDEXES {
        let handle = task::spawn(fetch_url(client.clone(), url(index)));
        handles.push(handle);
    }

    let mut tickers: HashSet<String> = HashSet::new();
    for handle in handles {
        // `.await` on the task to retrieve the result from the spawn
        let result = handle.await?.unwrap(); // This unwraps the result of the task
        tickers.extend(result.into_iter());
    }

    Ok(tickers)
}
