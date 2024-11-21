// src/download.rs

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

    // Fetch the list of versions
    let versions: VersionResponse = client.get(versions_url).send().await?.json().await?;

    // Get the latest version, default to an empty string if the list is empty
    let version = versions.0.first().cloned().unwrap_or_default();
    Ok(version)
}

pub async fn get_download_url() -> Result<String, reqwest::Error> {
    // Get the latest version
    let version = get_latest_version().await?;

    // Construct the download URL using the version
    let dl_url = format!("https://ddragon.leagueoflegends.com/cdn/dragontail-{}.tgz", version);
    Ok(dl_url)
}

pub async fn download_and_extract() -> Result<(), Box<dyn Error>> {
    // Fetch the download URL using the latest version
    let download_url = get_download_url().await?;

    // Download the file
    let response = reqwest::get(&download_url).await?;

    // Define the path where the .tgz file will be saved
    let file_path = Path::new("dragontail.tgz");

    // Create a file and save the downloaded content
    let mut file = File::create(file_path)?;
    let mut content = response.bytes().await?;

    // Write the content to the file
    file.write_all(&content)?;

    // Now let's extract the tar.gz file
    let file = File::open(file_path)?; // Open the saved file
    let decoder = GzDecoder::new(file); // Create a decoder for the gzipped content
    let mut archive = Archive::new(decoder);

    // Create the target directory to extract the files into
    let extract_path = Path::new("data");
    create_dir_all(extract_path)?; // Ensure the directory exists

    // Extract the archive to the specified directory
    archive.unpack(extract_path)?;

    Ok(())
}
