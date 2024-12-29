use futures::future::join_all;
use json::JsonValue;
use reqwest::Client;
use std::error::Error;
use std::time::Duration;

pub const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64)";

/*
Takes all the tasks awaits for them and filter unnecessary nested objects
*/
pub async fn handle_requests(
    handles: Vec<tokio::task::JoinHandle<Result<Option<JsonValue>, Box<dyn Error + Send + Sync>>>>,
) -> Result<Vec<JsonValue>, Box<dyn Error + Sync + Send>> {
    let raw_data = join_all(handles).await;

    println!("test 3");
    // Filter Nones, Result and Option wrappers
    let filtered_data: Vec<JsonValue> = raw_data
        .into_iter()
        .filter_map(|outer| outer.ok()?.ok()?.map(|value| value))
        .collect();

    Ok(filtered_data)
}

/*
Fetch asynchronously 1 URL and parses it
*/
pub async fn fetch_url(
    client: Client,
    url: String,
) -> Result<Option<JsonValue>, Box<dyn Error + Send + Sync>> {
    let response: JsonValue = match client
        .get(url)
        //.timeout(Duration::from_secs(5))
        .header(reqwest::header::USER_AGENT, USER_AGENT)
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
