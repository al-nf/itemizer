// src/download.rs
use reqwest::Client;
use rusqlite::{Connection, Result};
use tokio::fs::File as TokioFile;
use tokio::io::AsyncWriteExt;
use flate2::read::GzDecoder;
use tar::Archive;
use serde_json::Value;
use std::path::Path;
use std::io::{self, Read};

pub async fn download_file_and_store(conn: &Connection, version: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Check if the data for this version is already in the database
    if let Some(_) = get_tgz_data(conn, version)? {
        return Ok("Data already cached".to_string());
    }

    // URL for downloading the file
    let download_url = format!(
        "https://ddragon.leagueoflegends.com/cdn/dragontail-{}.tgz",
        version
    );

    let client = Client::new();
    let response = client.get(&download_url).send().await?;

    if !response.status().is_success() {
        return Err(format!("Failed to fetch file: HTTP status {}", response.status()).into());
    }

    // Read the response body as bytes
    let content = response.bytes().await?;

    // Save the content to a temporary .tgz file (in memory or on disk)
    let filename = format!("dragontail-{}.tgz", version);
    let mut file = TokioFile::create(&filename).await?;
    file.write_all(&content).await?;

    // Now extract and store the JSON files
    let tgz_data = io::Cursor::new(content);  // Convert the bytes into a reader
    let decompressed = GzDecoder::new(tgz_data); // Decompress the .tgz
    let mut archive = Archive::new(decompressed);

    // Iterate through the files inside the .tgz archive
    for entry in archive.entries()? {
        let mut entry = entry?;
        let path = entry.path()?.to_str().unwrap_or_default().to_string();

        // Only process JSON files
        if path.ends_with(".json") {
            let mut json_content = String::new();
            entry.read_to_string(&mut json_content)?;

            // Parse JSON
            let json_data: Value = serde_json::from_str(&json_content)?;

            // Store each JSON object in SQLite
            store_json_data(conn, version, &path, &json_data)?;
        }
    }

    Ok(format!("File downloaded and JSON data stored for version {}", version))
}

// Function to store JSON data in SQLite
pub fn store_json_data(conn: &Connection, version: &str, file_path: &str, json_data: &Value) -> Result<()> {
    let json_str = serde_json::to_string(json_data)?;

    conn.execute(
        "INSERT INTO json_data (version, file_path, data) VALUES (?1, ?2, ?3)",
        params![version, file_path, json_str],
    )?;
    Ok(())
}

// Function to check if the .tgz data for this version already exists
pub fn get_tgz_data(conn: &Connection, version: &str) -> Result<Option<Vec<u8>>> {
    let mut stmt = conn.prepare("SELECT data FROM tgz_files WHERE version = ?1 LIMIT 1")?;
    let mut rows = stmt.query(params![version])?;
    
    if let Some(row) = rows.next()? {
        let data: Vec<u8> = row.get(0)?;
        Ok(Some(data))
    } else {
        Ok(None)
    }
}

