use std::{env, fs, io, path::PathBuf};

fn main() -> io::Result<()>
{
    // ==== Source & destination paths ====
    let source_dir = PathBuf::from("assets/icons");
    let home_dir = env::home_dir().unwrap().display().to_string();
    let target_dir = PathBuf::from(format!("{}/.config/hoyoplay-umu/assets", home_dir));
    let target_dir_bmp = PathBuf::from(format!("{}/.local/share/icons", home_dir));

    // ==== Create the destination directory ====
    fs::create_dir_all(&target_dir)?;

    // ==== Copy all image files recursively ====
    if source_dir.exists()
    {
        for entry in fs::read_dir(&source_dir)?
        {
            let entry = entry?;
            let path = entry.path();
            if path.is_file()
            {
                if let Some(ext) = path.extension()
                {
                    if matches!(ext.to_str(), Some("bmp")) && !fs::exists(target_dir_bmp.join(entry.file_name())).unwrap()
                    {
                        let file_name = entry.file_name();
                        let dest_path = target_dir_bmp.join(file_name);
                        fs::copy(&path, &dest_path)?;
                        println!("cargo:warning=📦 Copied {:?} → {:?}", path, dest_path);
                    }
                    if matches!(ext.to_str(), Some("png") | Some("svg") | Some("ico") | Some("jpg") | Some("jpeg")) && !fs::exists(target_dir.join(entry.file_name())).unwrap()
                    {
                        let file_name = entry.file_name();
                        let dest_path = target_dir.join(file_name);
                        fs::copy(&path, &dest_path)?;
                        println!("cargo:warning=📦 Copied {:?} → {:?}", path, dest_path);
                    }
                }
            }
        }
    }
    else
    {
        println!("cargo:warning=⚠️ No icons folder found at {:?}", source_dir);
    }

    // ==== Trigger rebuild when icons change ====
    println!("cargo:rerun-if-changed=icons");

    Ok(())
}
