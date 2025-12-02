use crate::{HOME_DIR, actions::buttons_actions::UNINSTALL_SUCCEEDED, system::proton_ge::TMPWORKINGDIRECTORY};
use std::fs;
use lazy_static::lazy_static;



lazy_static! 
{
    pub static ref HOYOUMU_DIRS: [String; 8] = 
    [
        format!("{}/.config/hoyoplay-umu", *HOME_DIR), 
        format!("{}/.config/hoyoplay-umu/ProtonLatest", *HOME_DIR), 
        format!("{}/.config/hoyoplay-umu/umu_config", *HOME_DIR), 
        format!("{}/.config/hoyoplay-umu/wine_prefix", *HOME_DIR), 
        format!("{}/.local/share/applications", *HOME_DIR), 
        format!("{}/.config/protonfixes/localfixes", *HOME_DIR), 
        format!("{}/.config/hoyoplay-umu/icons", *HOME_DIR),
        TMPWORKINGDIRECTORY.to_string()
    ];
    pub static ref HOYOUMU_FILES: [String; 9] = 
    [
        format!("{}/hoyoplay_icon.png", HOYOUMU_DIRS[6]), 
        format!("{}/umu_config.toml", HOYOUMU_DIRS[2]), 
        format!("{}/hoyoumu.py", HOYOUMU_DIRS[5]), 
        format!("{}/Hoyoplay_setup.exe", HOYOUMU_DIRS[0]), 
        format!("{}/version", HOYOUMU_DIRS[1]), 
        format!("{}/Hoyoplay_gamemode.desktop", HOYOUMU_DIRS[4]), 
        format!("{}/Hoyoplay.desktop", HOYOUMU_DIRS[4]), 
        format!("{}/drive_c/Program Files/HoYoPlay/launcher.exe", HOYOUMU_DIRS[3]), 
        format!("{}/umu_setup_config.toml", HOYOUMU_DIRS[2]), 
    ];
}



pub fn create_dirs()
{
    for path in &*HOYOUMU_DIRS
    {
        fs::create_dir_all(path).expect("❌ Failed While Creating Folders");
    }
    println!("✅ All directories created");
}

pub fn remove_dirs()
{
    for path in &*HOYOUMU_DIRS
    {
        if path == &HOYOUMU_DIRS[4] { continue };
        println!("📣 removing dir: {}", path);
        let _ = fs::remove_dir_all(path);
    }
    *UNINSTALL_SUCCEEDED.lock().unwrap() = Some(true);
    println!("✅ All directories removed");
}

pub fn remove_files()
{
    for file in &*HOYOUMU_FILES
    {
        println!("📣 removing file: {}", file);
        let _ = fs::remove_file(file);
    }
    println!("✅ All files removed");
}
