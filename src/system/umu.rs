use crate::system::file_and_dirs::HOYOUMU_FILES;
use indoc::indoc;
use std::{env, fs, path::Path};

pub fn check_umu() -> String
{
    let app_name = "umu-run";
    let mut found = false;
    let mut path_found = String::new();

    if let Some(paths) = env::var_os("PATH")
    {
        for path in env::split_paths(&paths)
        {
            let full_path = path.join(app_name);
            if Path::new(&full_path).exists()
            {
                found = true;
                path_found = full_path.display().to_string();
                break;
            }
        }
    }

    if found
    {
        println!("✅ '{}' exists in {}", app_name, path_found);
        path_found
    }
    else
    {
        panic!("{app_name} is not installed, please install it first!!!");
    }
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
