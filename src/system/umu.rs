use crate::{system::file_and_dirs::HOYOUMU_FILES, HOME_DIR};
use indoc::indoc;
use lazy_static::lazy_static;
use std::{env, fs, path::Path};

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

pub fn check_umu() -> String
{
    let app_name = "umu-run";
    // 1. iterate through our known locations
    for candidate in &*UMU_RUN_PATHS 
    {
        if Path::new(candidate).exists() 
        {
            println!("✅ '{}' exists in {} | Checked With Candidate", app_name, candidate.to_string());
            return candidate.to_string();
        }
    }

    // 2. use PATH to see if it exists
    if let Some(paths) = env::var_os("PATH")
    {
        for path in env::split_paths(&paths)
        {
            let full_path = path.join(app_name);
            if Path::new(&full_path).exists()
            {
                println!("✅ '{}' exists in {} | Checked With PATH", app_name, full_path.display().to_string());
                return full_path.display().to_string();
            }
        }
    }

    panic!("{app_name} is not installed, please install it first!!!");
}


pub fn create_umu_config()
{
    if !fs::exists(&HOYOUMU_FILES[1]).unwrap()
    {
        // ==== Create the umu config content ====
        let from = String::from("'");
        let to = String::from('"');
        let content = indoc! {r#"
[umu]
prefix = '~/.config/hoyoplay-umu/wine_prefix'
proton = '~/.config/hoyoplay-umu/ProtonLatest'
game_id = 'hoyoumu'
exe = '~/.config/hoyoplay-umu/wine_prefix/drive_c/Program Files/HoYoPlay/launcher.exe'

[env]
PROTON_VERB = 'waitforexitandrun'
"#}
        .replace(&from, &to);

        fs::write(&HOYOUMU_FILES[1], content).unwrap();
        println!("✅ Created UMU config in: {}", &HOYOUMU_FILES[1]);
    }
    else
    {
        println!("✅ UMU config already created in: {}", &HOYOUMU_FILES[1]);
    }
}
