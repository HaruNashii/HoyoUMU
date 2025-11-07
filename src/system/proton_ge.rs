use flate2::read::GzDecoder;
use reqwest::*;
use serde_json::Value;
use std::{
    fs::{self, File},
    io::Cursor,
    path::{Path, PathBuf} 
};
use tar::Archive;
use crate::{actions::buttons_actions::GITHUB_API_AVAILABLE, system::file_and_dirs::{HOYOUMU_DIRS, HOYOUMU_FILES}};

const TMPWORKINGDIRECTORY: &str = "/tmp/proton-ge-custom";

async fn setup_client_for_proton_ge() -> (Client, String)
{
    // GitHub API URL
    let url = "https://api.github.com/repos/GloriousEggroll/proton-ge-custom/releases/latest";

    // Make the GET request (GitHub requires a User-Agent)
    let client = Client::new();
    let response = client.get(url).header("User-Agent", "rust-reqwest").send().await.unwrap().text().await.unwrap();
    //println!("✅ Client Created");
    (client, response)
}

async fn get_online_data(response: &str) -> (String, String, String)
{
    // Parse JSON
    let json: Value = serde_json::from_str(response).unwrap();

    // Extract the file name of the .tar.gz asset
    let mut received_name = String::new();
    let mut received_version = String::new();
    if let Some(assets) = json["assets"].as_array()
    {
        for asset in assets
        {
            if let Some(name) = asset["name"].as_str()
                && name.ends_with(".tar.gz")
            {
                received_name = name.to_string();
                received_version = name.replace(".tar.gz", "");
            }
        }
    }

    // Find the .tar.gz download URL
    let mut received_url = String::new();
    if let Some(assets) = json["assets"].as_array()
    {
        for asset in assets
        {
            if let Some(url) = asset["browser_download_url"].as_str()
                && url.ends_with(".tar.gz")
            {
                received_url = url.to_string();
            }
        }
    }

    if !received_url.is_empty() && !received_name.is_empty()
    {
        //println!("✅ Online Data Received");
        return (received_name, received_version, received_url);
    }
    panic!("No .tar.gz file found in the release.");
}

pub fn check_if_proton_ge_exist(option_received_version: Option<&String>, only_latest: bool) -> bool
{
    let received_version = if let Some(result) = option_received_version
    {
        result
    }
    else
    {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let (_, response) = rt.block_on(setup_client_for_proton_ge());

        // Parse JSON
        let json: Value = serde_json::from_str(&response).unwrap();

        // Extract the file name of the .tar.gz asset
        let mut received_version = String::new();
        if let Some(assets) = json["assets"].as_array()
        {
            for asset in assets
            {
                if let Some(name) = asset["name"].as_str()
                    && name.ends_with(".tar.gz")
                {
                    received_version = name.replace(".tar.gz", "");
                }
            }
        }
        &received_version.to_string()
    };

    let file_path = &HOYOUMU_FILES[4];
    if fs::exists(file_path).unwrap()
    {
        let content = fs::read_to_string(file_path).unwrap();

        // ===== Get string after the first space =====
        if let Some((_, after_space)) = content.split_once(' ')
        {
            let version_from_file = after_space.trim(); // remove extra spaces/newlines
            // ===== Compare with another string =====
            if version_from_file == received_version
            {
                println!("✅ Your Proton-GE Version: '{}' Is Already The Latest", received_version);
                return true;
            }
            else if unsafe{GITHUB_API_AVAILABLE == Some(true)}
            {
                    // ==== Remove ProtonLatest if existing version is older ====
                    if fs::exists(&HOYOUMU_DIRS[1]).unwrap() { fs::remove_dir_all(&HOYOUMU_DIRS[1]).unwrap(); };
                    println!("✅ Removed old {}, to install the new {}", version_from_file, received_version);
            }
            else 
            {
                if only_latest { return false; };
                println!("✅ Github API Unavailable, So Using Your Proton-GE Version: '{}'", received_version);
                return true;
            };
        }
    }
    else
    {
        println!("⚠️ Valid Proton-GE Installation Not Found, Downloading New One...");
    }
    false 
}

async fn download_online_data(client: Client, received_version: &String, received_url: &String)
{
    let file_path = &HOYOUMU_DIRS[1];

    // ===== Read file content =====
    if check_if_proton_ge_exist(Some(received_version), true)
    {
        return;
    };

    // ==== Download tarball ====
    let archive_path = format!("{}/{}", TMPWORKINGDIRECTORY, "ProtonLatest.tar.gz");
    if !fs::exists(&archive_path).unwrap()
    {
        let bytes = client.get(received_url).header("User-Agent", "rust-reqwest").send().await.unwrap().bytes().await.unwrap();
        let mut file = File::create(&archive_path).unwrap();
        std::io::copy(&mut Cursor::new(bytes), &mut file).unwrap();
        println!("✅ Downloaded {} to: {}", received_version, archive_path);
    }
    else
    {
        println!("✅ {} Already Exists In TMP Folder, Skipping Download...", received_version);
    };

    // ==== Extract tarball ====
    let extracted_dir = fs::read_dir(TMPWORKINGDIRECTORY).unwrap().filter_map(|e| e.ok()).find(|entry| entry.file_name().to_string_lossy().starts_with("GE-"));
    let extracted_dir_path: PathBuf = if let Some(dir_entry) = extracted_dir
    {
        println!("✅ {} Already Extracted, Skipping Extraction...", received_version);
        dir_entry.path()
    }
    else
    {
        let tar_gz = File::open(&archive_path).unwrap();
        let tar = GzDecoder::new(tar_gz);
        let mut archive = Archive::new(tar);
        archive.unpack(TMPWORKINGDIRECTORY).unwrap();
        println!("✅ Extracted {} into: {}", received_version, TMPWORKINGDIRECTORY);
        fs::read_dir(TMPWORKINGDIRECTORY).unwrap().filter_map(|e| e.ok()).find(|entry| entry.file_name().to_string_lossy().starts_with("GE-")).unwrap().path()
    };

    // ==== Move extracted folder ====
    let source = Path::new(&extracted_dir_path);
    let destination = Path::new(&file_path);
    fs::create_dir_all(destination).unwrap();

    let mut folders: Vec<(PathBuf, PathBuf)> = vec![(source.to_path_buf(), destination.to_path_buf())];
    let mut index = 0;

    while index < folders.len()
    {
        let (src, dst) = folders[index].clone();
        if src.exists()
        {
            for entry in fs::read_dir(&src).unwrap()
            {
                let entry = entry.unwrap();
                let from = entry.path();
                let to = dst.join(entry.file_name());
                if from.is_dir()
                {
                    fs::create_dir_all(&to).unwrap();
                    folders.push((from, to));
                }
                else
                {
                    fs::copy(&from, &to).unwrap();
                }
            }
        }
        index += 1;
    }
    println!("✅ Moved {} to {}", source.display(), destination.display());

    // ==== Remove tmp directory ====
    fs::remove_dir_all(TMPWORKINGDIRECTORY).unwrap();
}

pub fn download_proton_ge()
{
    // ==== create tokio runtime ====
    let rt = tokio::runtime::Runtime::new().unwrap();

    // ==== create dirs ====
    fs::create_dir_all(TMPWORKINGDIRECTORY).unwrap();

    // ==== online Variables ====
    let (client, response) = rt.block_on(setup_client_for_proton_ge());
    let (_proton_ge_file_name, proton_ge_version, proton_ge_download_url) = rt.block_on(get_online_data(&response));
    rt.block_on(download_online_data(client, &proton_ge_version, &proton_ge_download_url));

    // ==== Debug ====
    //println!("\n # ==== Debug Start ==== #");
    //println!("tmp_working_directory:  {}", TMPWORKINGDIRECTORY);
    //println!("proton_ge_file_name:    {}", proton_ge_file_name);
    //println!("proton_ge_version:      {}", proton_ge_version);
    //println!("proton_ge_download_url: {}", proton_ge_download_url);
    //println!("proton_ge_expect_file:  {}", &*PROTON_GE_EXPECT_FILE);
    //println!("# ==== Debug End ==== #\n");
}
