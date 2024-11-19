// src/version.rs

use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct VersionResponse(Vec<String>);

pub async fn get_latest_version() -> Result<String, reqwest::Error> {
    let client = Client::new();
    let versions_url = "https://ddragon.leagueoflegends.com/api/versions.json";
    
    let versions: VersionResponse = client.get(versions_url).send().await?.json().await?;
    
    Ok(versions.0.first().cloned().unwrap_or_default())
}

