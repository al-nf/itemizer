// src/version.rs
use rusqlite::{params, Connection, Result};
use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct VersionList(Vec<String>);

const DB_PATH: &str = "version_cache.db";

// Initialize the SQLite database and create the table for storing JSON data
pub fn init_db() -> Result<Connection> {
    let conn = Connection::open(DB_PATH)?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS json_data (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            version TEXT NOT NULL,
            file_path TEXT NOT NULL,
            data TEXT NOT NULL
        )",
        params![],
    )?;
    Ok(conn)
}

// Retrieve the cached version from SQLite
pub fn get_cached_version(conn: &Connection) -> Result<Option<String>> {
    let mut stmt = conn.prepare("SELECT version FROM version_cache ORDER BY id DESC LIMIT 1")?;
    let mut rows = stmt.query(params![])?;
    
    if let Some(row) = rows.next()? {
        let version: String = row.get(0)?;
        Ok(Some(version))
    } else {
        Ok(None)
    }
}

// Cache the latest version in SQLite
pub fn cache_version(conn: &Connection, version: &str) -> Result<()> {
    conn.execute("INSERT INTO version_cache (version) VALUES (?1)", params![version])?;
    Ok(())
}

// Fetch the latest version from the API or SQLite cache
pub async fn fetch_latest_version(conn: &Connection) -> Result<String> {
    if let Some(cached_version) = get_cached_version(conn)? {
        return Ok(cached_version); // Return cached version if available
    }

    // Fetch the latest version from the API
    let version_url = "https://ddragon.leagueoflegends.com/api/versions.json";
    let client = Client::new();
    let versions: VersionList = client.get(version_url)
        .send().await?
        .json().await?;

    let latest_version = versions.0.first().ok_or_else(|| {
        rusqlite::Error::InvalidQuery("No versions found".into())
    })?;

    // Cache the latest version in the database
    cache_version(conn, latest_version)?;

    Ok(latest_version.to_string())
}

