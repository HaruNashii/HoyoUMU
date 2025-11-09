use crate::{HOME_DIR, system::file_and_dirs::HOYOUMU_FILES};
use std::{env, fs, path::Path};
use lazy_static::lazy_static;
use indoc::indoc;



lazy_static! 
{
    pub static ref UMU_RUN_PATHS: [String; 7] = 
    [
        format!("{}/.cargo/bin/umu-run", *HOME_DIR), 
        format!("{}/.local/bin/umu-run", *HOME_DIR), 
        "/usr/bin/umu-run".to_string(), 
        "/usr/local/bin/umu-run".to_string(), 
        "/home/beth/.cargo/bin/umu-run".to_string(), 
        "/usr/local/bin/umu-run".to_string(), 
        "/usr/bin/umu-run".to_string()
    ];
}



pub fn check_umu() -> Option<String>
{
    // 1. iterate through our known locations
    for candidate in &*UMU_RUN_PATHS
    {
        if Path::new(candidate).exists()
        {
            println!("✅ 'umu-run' exists in {} | Checked With Candidate", candidate);
            return Some(candidate.to_string());
        }
    }

    // 2. use PATH to see if it exists
    if let Some(paths) = env::var_os("PATH")
    {
        for path in env::split_paths(&paths)
        {
            let full_path = path.join("umu-run");
            if Path::new(&full_path).exists()
            {
                println!("✅ 'umu-run' exists in {} | Checked With PATH", full_path.display());
                return Some(full_path.display().to_string());
            }
        }
    }

    None
}


pub fn create_umu_config()
{
    if !fs::exists(&HOYOUMU_FILES[1]).unwrap()
    {
        // ==== Create the umu config content ====
        let from = String::from("'");
        let to = String::from('"');


//this have weird readability but there is nothing i can do :(
let content = indoc! 
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


        fs::write(&HOYOUMU_FILES[1], content).unwrap();
        println!("✅ Created UMU config in: {}", &HOYOUMU_FILES[1]);
    }
    else
    {
        println!("✅ UMU config already created in: {}", &HOYOUMU_FILES[1]);
    }
}
