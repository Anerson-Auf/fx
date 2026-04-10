use sevenz_rust::decompress_file;
use rfd::FileDialog;
use color_eyre::{eyre::Result};
use reqwest::blocking::Client;
use std::io::{Read, Write};
use std::fs::{File, create_dir_all};
use std::path::Path;
use std::time::{Duration, Instant};

const URLS: [(&str, &str); 3] = [
    ("https://github.com/Anerson-Auf/fx/releases/download/q/ToWRR.7z", ""), // game folder
    ("https://github.com/Anerson-Auf/fx/releases/download/q/ToW_Data.7z", "ToW_Data"), // game folder
    ("https://github.com/Anerson-Auf/fx/releases/download/q/StreamingAssets.7z", "ToW_Data/StreamingAssets") // game folder/StreamingAssets
];


fn main() -> Result<()> {
    let paths = ["C:/lua_dump", "C:/lua_dump_all"];
    for p in paths {
        let path = Path::new(p);
        if !path.exists() {
            create_dir_all(path)?;
        }
    }
    let game_path = FileDialog::new().pick_folder().unwrap();
    let client = Client::new();
    let mut last_update = Instant::now();
    for (url, sub_path) in URLS {
        let mut resp = client.get(url).send()?.error_for_status()?;
        
        let dest_dir = game_path.join(sub_path);
        create_dir_all(&dest_dir)?;
        
        let archive_path = dest_dir.join("temp.7z");
        
        let mut file = File::create(&archive_path)?;

        let total_size = resp.content_length().unwrap_or(0);
        let mut downloaded: u64 = 0;
        
        let mut buffer = [0; 8192];
        loop {
            let n = resp.read(&mut buffer)?;
            if n == 0 {
                break;
            }
            file.write_all(&buffer[..n])?;
            downloaded += n as u64;
            if total_size > 0 && last_update.elapsed() >= Duration::from_millis(100) {
                print!("\r[{:?}] Progress: {:.2}%", sub_path, (downloaded as f64 / total_size as f64) * 100.0);
                std::io::stdout().flush().unwrap();
                last_update = Instant::now();
            }
        }
        if downloaded == total_size {
            println!();
            println!("[{:?}] Successfully downloaded.", sub_path);
        }
        //copy(&mut resp, &mut file)?;

        decompress_file(&archive_path, &dest_dir)?;
        println!("[{:?}] Successfully decompress.", sub_path);

        std::fs::remove_file(&archive_path)?;
    }

    Ok(())
}
