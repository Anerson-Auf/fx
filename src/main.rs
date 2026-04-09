use sevenz_rust::decompress_file;
use rfd::FileDialog;
use color_eyre::{eyre::Result};
use reqwest::blocking::Client;
use std::io::{copy};
use std::fs::{File, create_dir_all};

const URLS: [(&str, &str); 3] = [
    ("https://github.com/Anerson-Auf/fx/releases/download/q/ToWRR.7z", ""), // game folder
    ("https://github.com/Anerson-Auf/fx/releases/download/q/ToW_Data.7z", "ToW_Data"), // game folder
    ("https://github.com/Anerson-Auf/fx/releases/download/q/StreamingAssets.7z", "ToW_Data/StreamingAssets") // game folder/StreamingAssets
];

fn main() -> Result<()> {
    let game_path = FileDialog::new().pick_folder().unwrap();
    let client = Client::new();

    for (url, sub_path) in URLS {
        let mut resp = client.get(url).send()?.error_for_status()?;

        let dest_dir = game_path.join(sub_path);
        create_dir_all(&dest_dir)?;

        let archive_path = dest_dir.join("temp.7z");

        let mut file = File::create(&archive_path)?;
        copy(&mut resp, &mut file)?;

        decompress_file(&archive_path, &dest_dir)?;

        std::fs::remove_file(&archive_path)?;
    }

    Ok(())
}