/*
 * File: champion.rs
 * Copyright: 2024, Alan Fung
 * Description: returns champion.json as an http response
 */
use actix_web::{web, HttpResponse, Responder};
use reqwest::Client;
use serde_json::Value;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

// URL to fetch champion data
const CHAMPIONS_URL: &str = "https://cdn.merakianalytics.com/riot/lol/resources/latest/en-US/champions.json";
const CACHE_PATH: &str = "champs_cache.json";

// Ensure the cache file exists
pub async fn ensure_cache() -> Result<(), String> {
    if Path::new(CACHE_PATH).exists() {
        return Ok(()); // Cache already exists
    }

    // Fetch data and create the cache file
    let client = Client::new();
    let response = client.get(CHAMPIONS_URL).send().await.map_err(|_| "Failed to fetch data")?;

    if response.status().is_success() {
        let body = response.text().await.map_err(|_| "Failed to read body")?;
        let mut file = File::create(CACHE_PATH).map_err(|_| "Failed to create cache file")?;
        file.write_all(body.as_bytes()).map_err(|_| "Failed to write to cache file")?;
        Ok(())
    } else {
        Err("Failed to fetch data from source".to_string())
    }
}

// Fetch champions manually (used in /fetch-champs route)
pub async fn fetch_champs() -> impl Responder {
    match ensure_cache().await {
        Ok(_) => HttpResponse::Ok().body("Cache successfully created or already exists"),
        Err(err) => HttpResponse::InternalServerError().body(err),
    }
}

// Get a champion by name
pub async fn get_champion(name: web::Path<String>) -> impl Responder {
    // Ensure the cache exists (in case it was deleted during runtime)
    if let Err(err) = ensure_cache().await {
        return HttpResponse::InternalServerError().body(err);
    }

    // Read the JSON file
    let data = match fs::read_to_string(CACHE_PATH) {
        Ok(content) => content,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to read cache file"),
    };

    // Parse the JSON content
    let champs: Value = match serde_json::from_str(&data) {
        Ok(parsed) => parsed,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to parse JSON file"),
    };

    // Search for the champion
    if let Some(champion) = champs.get(&name.into_inner()) {
        HttpResponse::Ok().json(champion)
    } else {
        HttpResponse::NotFound().body("Champion not found")
    }
}

