/*
 * File: download.rs
 * Copyright: 2024, Alan Fung
 * Description: downloads the latest stats from the League of Legends API
 */
use reqwest::Client;
use serde::Deserialize;
use flate2::read::GzDecoder;
use tar::Archive;
use std::path::Path;
use std::error::Error;
use std::fs::{self, File, create_dir_all};
use std::io::{self, Read, Write};

static VERSION_FILE: &str = "version.txt";

#[derive(Deserialize)]
pub struct VersionResponse(Vec<String>);

pub async fn get_latest_version() -> Result<String, reqwest::Error> {
    let client = Client::new();
    let versions_url = "https://ddragon.leagueoflegends.com/api/versions.json";
    let versions: VersionResponse = client.get(versions_url).send().await?.json().await?;
    Ok(versions.0.first().cloned().unwrap_or_default())
}

pub async fn get_download_url() -> Result<String, reqwest::Error> {
    let version = get_latest_version().await?;
    Ok(format!("https://ddragon.leagueoflegends.com/cdn/dragontail-{}.tgz", version))
}

fn read_current_version() -> io::Result<String> {
    if Path::new(VERSION_FILE).exists() {
        let mut version = String::new();
        File::open(VERSION_FILE)?.read_to_string(&mut version)?;
        Ok(version.trim().to_string())
    } else {
        Ok(String::new())
    }
}

fn write_current_version(version: &str) -> io::Result<()> {
    let mut file = File::create(VERSION_FILE)?;
    file.write_all(version.as_bytes())?;
    Ok(())
}

pub async fn check_and_update() -> Result<(), Box<dyn Error>> {
    let latest_version = get_latest_version().await?;

    let current_version = read_current_version()?;
    if current_version == latest_version {
        return Ok(()); 
    }

    let download_url = get_download_url().await?;
    let response = reqwest::get(&download_url).await?;
    let file_path = Path::new("dragontail.tgz");

    let mut file = File::create(file_path)?;
    let content = response.bytes().await?;
    file.write_all(&content)?;

    if Path::new("data").exists() {
        fs::remove_dir_all("data")?; 
    }
    let file = File::open(file_path)?;
    let decoder = GzDecoder::new(file);
    let mut archive = Archive::new(decoder);
    create_dir_all("data")?;
    archive.unpack("data")?;

    write_current_version(&latest_version)?;

    Ok(())
}
