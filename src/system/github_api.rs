use reqwest::Client;
use serde_json::Value;

pub async fn fetch_github_api(url: &str) -> Result<Value, Box<dyn std::error::Error>>
{
    let client = Client::new();
    let res = client.get(url).header("User-Agent", "rust-reqwest").send().await?;
    let text = client.get(url).send().await?.text().await?;

    // Check for rate-limit headers
    if let Some(remaining) = res.headers().get("x-ratelimit-remaining")
        && remaining == "0"
    {
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
            eprintln!("❌ Rate limit reached! Switching logic...");
            // handle rate limit here, e.g.:
            // use fallback token, cached data, or skip
            return false;
        }
        Err(e) =>
        {
            eprintln!("❌ Request failed: {}", e);
            return false;
        }
    }
}
