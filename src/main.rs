use rusqlite::{Connection, Result, params};
use serde_json::Value;
use std::error::Error;

pub fn get_json_data(conn: &Connection, version: &str, file_path: &str) -> Result<Option<Value>, Box<dyn Error>> {
    // Prepare the SQL query to select the JSON data by version and file path
    let mut stmt = conn.prepare("SELECT data FROM json_data WHERE version = ?1 AND file_path = ?2 LIMIT 1")?;
    
    // Execute the query and get the result
    let mut rows = stmt.query(params![version, file_path])?;
    
    // Check if any row is returned
    if let Some(row) = rows.next()? {
        let json_str: String = row.get(0)?;
        let json_data: Value = serde_json::from_str(&json_str)?;
        Ok(Some(json_data))  // Return the parsed JSON data
    } else {
        Ok(None)  // Return None if no data is found
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Open the SQLite database (it should already exist)
    let conn = Connection::open("version_cache.db")?;

    // Query for a specific version and file path
    let version = "14.22.1";  // Example version
    let file_path = "14.22.1/data/en_US/champion.json";  // Example file path

    // Retrieve the JSON data from the database
    match get_json_data(&conn, version, file_path)? {
        Some(json_data) => {
            // Print the JSON data
            println!("Found JSON data: {:?}", json_data);
        },
        None => {
            // Handle case where the file path isn't found
            println!("No data found for version '{}' and file path '{}'", version, file_path);
        }
    }

    Ok(())
}

