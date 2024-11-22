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

const CHAMPIONS_URL: &str = "https://cdn.merakianalytics.com/riot/lol/resources/latest/en-US/champions.json";
const CACHE_PATH: &str = "champs_cache.json";

pub async fn ensure_cache() -> Result<(), String> {
    if Path::new(CACHE_PATH).exists() {
        return Ok(()); 
    }

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

pub async fn fetch_champs() -> impl Responder {
    match ensure_cache().await {
        Ok(_) => HttpResponse::Ok().body("Cache successfully created or already exists"),
        Err(err) => HttpResponse::InternalServerError().body(err),
    }
}

pub async fn get_champion(name: web::Path<String>) -> impl Responder {
    if let Err(err) = ensure_cache().await {
        return HttpResponse::InternalServerError().body(err);
    }

    let data = match fs::read_to_string(CACHE_PATH) {
        Ok(content) => content,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to read cache file"),
    };

    let champs: Value = match serde_json::from_str(&data) {
        Ok(parsed) => parsed,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to parse JSON file"),
    };

    if let Some(champion) = champs.get(&name.into_inner()) {
        HttpResponse::Ok().json(champion)
    } else {
        HttpResponse::NotFound().body("Champion not found")
    }
}

pub async fn get_champion_property_nested(path: web::Path<(String, String)>) -> impl Responder {
    let cache_path = "champs_cache.json";

    let data = match fs::read_to_string(cache_path) {
        Ok(content) => content,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to read cache file"),
    };

    let champs: Value = match serde_json::from_str(&data) {
        Ok(parsed) => parsed,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to parse JSON file"),
    };

    let (champion_name, property) = path.into_inner();
    if let Some(champion) = champs.get(&champion_name) {
        let keys: Vec<&str> = property.split('.').collect();
        let mut current_value = champion;

        for key in keys {
            if let Some(value) = current_value.get(key) {
                current_value = value;
            } else {
                return HttpResponse::NotFound().body(format!("Property '{}' not found", property));
            }
        }

        return HttpResponse::Ok().json(current_value);
    }

    HttpResponse::NotFound().body("Champion not found")
}
