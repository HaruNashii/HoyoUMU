use crate::system::file_and_dirs::HOYOUMU_FILES;
use std::{fs, path::Path};
use indoc::indoc;



pub fn create_proton_fixes()
{
    if let Some(parent) = Path::new(&HOYOUMU_FILES[2]).parent() 
    {
        fs::create_dir_all(parent).unwrap();
    }

    if !fs::exists(&HOYOUMU_FILES[2]).unwrap()
    {


//this have weird readability but there is nothing i can do :(
let content = indoc! 
{
r#"
from protonfixes import util

def main() -> None:
    util.set_environment('UMU_USE_STEAM', '1')
    util.set_game_drive(True)
"#
};


        fs::write(&HOYOUMU_FILES[2], content).unwrap();
        println!("✅ Created protonfix in: {}", &HOYOUMU_FILES[2]);
    }
    else
    {
        println!("✅ Protonfix already created in: {}", &HOYOUMU_FILES[2]);
    }
}
