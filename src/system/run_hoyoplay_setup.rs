use crate::system::file_and_dirs::HOYOUMU_FILES;
use std::{
    fs,
    path::Path,
    process::{Command, Stdio}
};


pub fn run_hoyoplay_setup(path_to_umu_run: &String)
{
    if fs::exists(Path::new(&HOYOUMU_FILES[8])).unwrap()
    {
        println!("✅ Hoyoplay is already installed!");
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
