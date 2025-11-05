use crate::HOME_DIR;
use lazy_static::lazy_static;
use std::fs;

lazy_static! {
    pub static ref HOYOUMU_DIRS: [String; 7] = [format!("{}/.config/hoyoplay-umu", *HOME_DIR), format!("{}/.config/hoyoplay-umu/ProtonLatest", *HOME_DIR), format!("{}/.config/hoyoplay-umu/umu_config", *HOME_DIR), format!("{}/.config/hoyoplay-umu/assets", *HOME_DIR), format!("{}/.config/hoyoplay-umu/wine_prefix", *HOME_DIR), format!("{}/.local/share/applications", *HOME_DIR), format!("{}/.config/protonfixes/localfixes", *HOME_DIR)];
    pub static ref HOYOUMU_FILES: [String; 10] = [format!("{}/hoyoplay_icon.png", HOYOUMU_DIRS[3]), format!("{}/umu_config.toml", HOYOUMU_DIRS[2]), format!("{}/hoyoumu.py", HOYOUMU_DIRS[6]), format!("{}/Hoyoplay_setup.exe", HOYOUMU_DIRS[0]), format!("{}/version", HOYOUMU_DIRS[1]), format!("{}/Hoyoplay_gamemode.desktop", HOYOUMU_DIRS[5]), format!("{}/Hoyoplay.desktop", HOYOUMU_DIRS[5]), format!("{}/hoyoumu_icon.png", HOYOUMU_DIRS[3]), format!("{}/drive_c/Program Files/HoYoPlay/launcher.exe", HOYOUMU_DIRS[4]), format!("{}/warning.png", HOYOUMU_DIRS[3])];
}

pub fn create_dirs()
{
    for path in &*HOYOUMU_DIRS
    {
        fs::create_dir_all(path).expect("Failed While Creating Folders");
    }
    println!("✅ All directories created");
}

pub fn remove_dirs()
{
    for (index, path) in HOYOUMU_DIRS.iter().enumerate()
    {
        if index < 3
        {
            println!("removing dir: {}", path);
            fs::remove_dir_all(path).expect("Failed While Creating Folders");
        }
        else
        {
            return;
        }
    }
    println!("✅ All directories removed");
}

pub fn remove_files()
{
    for file in &*HOYOUMU_FILES
    {
        let _ = fs::remove_file(file);
    }
    println!("✅ All files removed");
}
