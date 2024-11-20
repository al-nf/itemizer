// src/download.rs

use reqwest::Client;
use serde::Deserialize;
use flate2::read::GzDecoder;
use tar::Archive;
use std::path::Path;
use std::error::Error;

#[derive(Deserialize)]
pub struct VersionResponse(Vec<String>);

pub async fn get_latest_patch() -> Result<String, reqwest::Error> {
    let client = Client::new();
    let versions_url = "https://ddragon.leagueoflegends.com/api/versions.json";
    
    let versions: VersionResponse = client.get(versions_url).send().await?.json().await?;
    let version = versions.0.first().cloned().unwrap_or_default();

    let dl_url = format!("https://ddragon.leagueoflegends.com/cdn/dragontail-{}.tgz", version);
    Ok(dl_url)
}

pub async fn download_and_extract() -> Result<(), Box<dyn Error>> {
    let download_url = get_latest_patch().await?;
    let response = reqwest::get(&download_url).await?;
    let body = response.bytes().await?;
    
    let decoder = GzDecoder::new(&body[..]);
    let mut archive = Archive::new(decoder);
    
    let extract_path = Path::new("data");
    archive.unpack(extract_path)?;

    //traverse_and_parse_files(extract_path).await?;
    
    Ok(())
}
