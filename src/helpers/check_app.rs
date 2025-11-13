use crate::HOME_DIR;
use std::{env, path::Path};
use lazy_static::lazy_static;



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
        let new_candidate = format!("{}/{}", candidate, app_name);
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

    println!("'{}'  Doesn't exist in PATH", app_name);
    None
}
