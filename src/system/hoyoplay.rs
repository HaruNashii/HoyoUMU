use crate::system::file_and_dirs::HOYOUMU_FILES;
use bytes::Bytes;
use reqwest::Client;
use std::{
    fs,
    path::Path,
    process::{Command, Stdio}
};

pub fn check_if_hoyoplay_exist() -> bool
{
    if fs::exists(Path::new(&HOYOUMU_FILES[8])).unwrap()
    {
        println!("✅ Hoyoplay is already installed!");
        return true;
    };
    false
}

pub fn run_hoyoplay_setup(path_to_umu_run: &String)
{
    if check_if_hoyoplay_exist()
    {
        return;
    };
    let command_str = format!("{} {}", path_to_umu_run, &HOYOUMU_FILES[3]);
    let hoyoplay_setup_status = Command::new("sh").arg("-c").arg(&command_str).stdout(Stdio::null()).stderr(Stdio::null()).status().expect("❌ Failed to execute Hoyoplay setup");
    if hoyoplay_setup_status.success()
    {
        println!("✅ Hoyoplay setup executed successfully!");
    }
    else
    {
        eprintln!("⚠️ Hoyoplay setup exited with status: {:?}", hoyoplay_setup_status.code());
    }
}

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
