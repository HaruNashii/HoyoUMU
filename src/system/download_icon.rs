use crate::system::file_and_dirs::HOYOUMU_FILES;
use bytes::Bytes;
use reqwest::Client;
use std::fs;



async fn setup_client_for_icon() -> Bytes
{
    let image_url = "https://act-webstatic.hoyoverse.com/puzzle/hyp/pz_Bur_m6Btc7/resource/puzzle/2024/10/21/b9992eaa38d4b36641accee82ede7bd3_3188319471762169697.png";
    let client = Client::builder().user_agent("HoyoUMU/1.0 (reqwest)").build().unwrap();
    client.get(image_url).send().await.unwrap().error_for_status().unwrap().bytes().await.unwrap()
}

#[tokio::main]
pub async fn download_icon()
{
    if !fs::exists(&HOYOUMU_FILES[0]).unwrap()
    {
        let response = setup_client_for_icon().await;
        tokio::fs::write(&HOYOUMU_FILES[0], response).await.unwrap();
        println!("✅ HoyoPlay icon downloaded in: {}", &HOYOUMU_FILES[0]);
    }
    else
    {
        println!("✅ HoyoPlay icon already created in: {}", &HOYOUMU_FILES[0]);
    }
}
