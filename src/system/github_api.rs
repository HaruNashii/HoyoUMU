use chrono::Local;
use reqwest::Client;
use serde_json::Value;

// Safe global mutable state
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    pub static ref GITHUB_API_TIME_RESET: Mutex<Option<String>> = Mutex::new(None);
}

pub async fn fetch_github_api(url: &str) -> Result<Value, Box<dyn std::error::Error>>
{
    let client = Client::new();
    let res = client.get(url).header("User-Agent", "rust-reqwest").send().await?;
    let text = client.get(url).send().await?.text().await?;

    // Check for rate-limit headers
    if let Some(remaining) = res.headers().get("x-ratelimit-remaining")
        && remaining == "0"
    {
        if let Some(reset_header) = res.headers().get("X-RateLimit-Reset")
        {
            let reset_unix = reset_header.to_str()?.parse::<i64>()?;
            let utc_time = chrono::DateTime::from_timestamp(reset_unix, 0).unwrap();
            let local_time = utc_time.with_timezone(&Local);
            let local_time_string = local_time.to_string();
            let parts: Vec<&str> = local_time_string.splitn(3, ' ').collect(); // split into at most 3 parts
            let local_time_to_send = if parts.len() >= 2 { format!("{} {}", parts[0], parts[1]) } else { local_time_string.to_string() };
            // Assign new reset time safely
            *GITHUB_API_TIME_RESET.lock().unwrap() = Some(local_time_to_send);
        }
        eprintln!("⚠️ GitHub API rate limit exceeded (via header)");
        return Err("Rate limit exceeded".into());
    }

    // Check status code
    if res.status().as_u16() == 403 || res.status().as_u16() == 429
    {
        if text.contains("API rate limit exceeded")
        {
            eprintln!("⚠️ GitHub API rate limit exceeded (via body)");
            return Err("Rate limit exceeded".into());
        }
        else
        {
            // other 403 errors, return content anyway
            return Err(format!("HTTP {}: {}", &res.status(), text).into());
        }
    }

    // Parse normally
    let json: Value = res.json().await?;
    Ok(json)
}

#[tokio::main]
pub async fn check_if_github_api_is_available() -> bool
{
    let url = "https://api.github.com/repos/GloriousEggroll/proton-ge-custom/releases/latest";
    match fetch_github_api(url).await
    {
        Ok(..) =>
        {
            return true;
        }
        Err(e) if e.to_string().contains("Rate limit exceeded") =>
        {
            return false;
        }
        Err(e) =>
        {
            eprintln!("❌ Request failed: {}", e);
            return false;
        }
    }
}
