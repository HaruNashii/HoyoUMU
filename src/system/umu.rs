use crate::{HOME_DIR, system::file_and_dirs::HOYOUMU_FILES};
use std::{env, fs, path::Path};
use lazy_static::lazy_static;
use indoc::indoc;



lazy_static! 
{
    pub static ref BIN_PATHS: [String; 7] = 
    [
        format!("{}/.cargo/bin", *HOME_DIR), 
        format!("{}/.local/bin", *HOME_DIR), 
        "/usr/bin".to_string(), 
        "/usr/local/bin".to_string(), 
        "/home/beth/.cargo/bin".to_string(), 
        "/usr/local/bin".to_string(), 
        "/usr/bin".to_string()
    ];
}



pub fn check_app_availability(app_name: String) -> Option<String>
{
    // 1. iterate through our known locations
    for candidate in &*BIN_PATHS
    {
        let new_candidate = format!("{}/umu-run", candidate);
        if Path::new(&new_candidate).exists()
        {
            println!("✅ '{}' exists in {} | Checked With Candidate", app_name, new_candidate);
            return Some(new_candidate.to_string());
        }
    }

    // 2. use PATH to see if it exists
    if let Some(paths) = env::var_os("PATH")
    {
        for path in env::split_paths(&paths)
        {
            let full_path = path.join(&app_name);
            if Path::new(&full_path).exists()
            {
                println!("✅ '{}' exists in {} | Checked With PATH", app_name, full_path.display());
                return Some(full_path.display().to_string());
            }
        }
    }

    None
}


pub fn create_umu_config()
{
    if !fs::exists(&HOYOUMU_FILES[8]).unwrap()
    {
        // ==== Create the umu config content ====
        let from = String::from("'");
        let to = String::from('"');

//this have weird readability but there is nothing i can do :(
let content_2 = indoc! 
{
r#"
[umu]
prefix = '~/.config/hoyoplay-umu/wine_prefix'
proton = '~/.config/hoyoplay-umu/ProtonLatest'
game_id = 'hoyoumu'
exe = '~/.config/hoyoplay-umu/Hoyoplay_setup.exe'

[env]
PROTON_VERB = 'waitforexitandrun'
"#
}.replace(&from, &to);

        fs::write(&HOYOUMU_FILES[8], content_2).unwrap();
        println!("✅ Created UMU Setup config in: {}", &HOYOUMU_FILES[8]);
    }
    else
    {
        println!("✅ UMU Setup config already created in: {}", &HOYOUMU_FILES[8]);
    }



    if !fs::exists(&HOYOUMU_FILES[1]).unwrap()
    {
        // ==== Create the umu config content ====
        let from = String::from("'");
        let to = String::from('"');

//this have weird readability but there is nothing i can do :(
let content_1 = indoc! 
{
r#"
[umu]
prefix = '~/.config/hoyoplay-umu/wine_prefix'
proton = '~/.config/hoyoplay-umu/ProtonLatest'
game_id = 'hoyoumu'
exe = '~/.config/hoyoplay-umu/wine_prefix/drive_c/Program Files/HoYoPlay/launcher.exe'

[env]
PROTON_VERB = 'waitforexitandrun'
"#
}.replace(&from, &to);

        fs::write(&HOYOUMU_FILES[1], content_1).unwrap();
        println!("✅ Created UMU config in: {}", &HOYOUMU_FILES[1]);
    }
    else
    {
        println!("✅ UMU config already created in: {}", &HOYOUMU_FILES[1]);
    }
}
