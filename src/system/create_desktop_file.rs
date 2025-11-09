use crate::system::file_and_dirs::HOYOUMU_FILES;
use std::fs;



pub fn create_desktop_file(path_to_umu: &String)
{
    if !fs::exists(&HOYOUMU_FILES[6]).unwrap()
    {


//this have weird readability but there is nothing i can do :(
let content = format!
(
"
[Desktop Entry]
Name=Hoyoplay
Comment=Launch Hoyoplay
Exec={} --config {}
Icon={}
Terminal=false
Type=Application
Categories=Game;
",
path_to_umu, 
HOYOUMU_FILES[1], 
HOYOUMU_FILES[0]
);


        fs::write(&HOYOUMU_FILES[6], content).unwrap();
        println!("✅ desktop file created in: {}", &HOYOUMU_FILES[6]);
    }
    else
    {
        println!("✅ desktop file already created in: {}", &HOYOUMU_FILES[6]);
    }
}
