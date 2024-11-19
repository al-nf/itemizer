// src/version.rs

use reqwest::Client;
use serde::Deserialize;

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

