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
use std::fs::File;
use std::io::Write;
use std::fs::create_dir_all;

#[derive(Deserialize)]
pub struct VersionResponse(Vec<String>);

pub async fn get_latest_version() -> Result<String, reqwest::Error> {
    let client = Client::new();
    let versions_url = "https://ddragon.leagueoflegends.com/api/versions.json";
    let versions: VersionResponse = client.get(versions_url).send().await?.json().await?;

    let version = versions.0.first().cloned().unwrap_or_default();
    Ok(version)
}

pub async fn get_download_url() -> Result<String, reqwest::Error> {
    let version = get_latest_version().await?;
    let dl_url = format!("https://ddragon.leagueoflegends.com/cdn/dragontail-{}.tgz", version);
    Ok(dl_url)
}

pub async fn download_and_extract() -> Result<(), Box<dyn Error>> {
    let download_url = get_download_url().await?;
    let response = reqwest::get(&download_url).await?;
    let file_path = Path::new("dragontail.tgz");

    let mut file = File::create(file_path)?;
    let mut content = response.bytes().await?;

    file.write_all(&content)?;

    let file = File::open(file_path)?;
    let decoder = GzDecoder::new(file);
    let mut archive = Archive::new(decoder);

    let extract_path = Path::new("data");
    create_dir_all(extract_path)?; 

    archive.unpack(extract_path)?;
    Ok(())
}
