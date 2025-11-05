use crate::system::file_and_dirs::HOYOUMU_FILES;
use bytes::Bytes;
use reqwest::Client;
use std::fs;

async fn setup_client_for_hoyoplay() -> Bytes
{
    let hoyoplay_url = "https://sg-public-api.hoyoverse.com/event/download_porter/trace/hyp_global/hyphoyoverse/default";
    let client = Client::builder().user_agent("HoyoUMU/1.0 (reqwest)").build().unwrap();
    client.get(hoyoplay_url).send().await.unwrap().error_for_status().unwrap().bytes().await.unwrap()
}

#[tokio::main]
pub async fn download_hoyoplay()
{
    if !fs::exists(&HOYOUMU_FILES[3]).unwrap()
    {
        let response = setup_client_for_hoyoplay().await;
        tokio::fs::write(&HOYOUMU_FILES[3], response).await.unwrap();
        println!("✅ HoyoPlay setup downloaded in: {}", &HOYOUMU_FILES[3]);
    }
    else
    {
        println!("✅ HoyoPlay already downloaded in {}", &HOYOUMU_FILES[3]);
    }
}
