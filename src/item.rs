/*
 * File: item.rs
 * Copyright: 2024, Alan Fung
 * Description: returns items or a specific item
 */
use actix_web::{web, HttpResponse, Responder};
use reqwest::Client;
use serde_json::Value;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

const ITEMS_URL: &str = "https://cdn.merakianalytics.com/riot/lol/resources/latest/en-US/items.json";
const CACHE_PATH: &str = "items_cache.json";

pub async fn ensure_cache() -> Result<(), String> {
    if Path::new(CACHE_PATH).exists() {
        return Ok(()); 
    }

    let client = Client::new();
    let response = client.get(ITEMS_URL).send().await.map_err(|_| "Failed to fetch data")?;

    if response.status().is_success() {
        let body = response.text().await.map_err(|_| "Failed to read body")?;
        let mut file = File::create(CACHE_PATH).map_err(|_| "Failed to create cache file")?;
        file.write_all(body.as_bytes()).map_err(|_| "Failed to write to cache file")?;
        Ok(())
    } else {
        Err("Failed to fetch data from source".to_string())
    }
}

pub async fn fetch_items() -> impl Responder {
    match ensure_cache().await {
        Ok(_) => HttpResponse::Ok().body("Cache successfully created or already exists"),
        Err(err) => HttpResponse::InternalServerError().body(err),
    }
}

pub async fn get_item(name: web::Path<String>) -> impl Responder {
    if let Err(err) = ensure_cache().await {
        return HttpResponse::InternalServerError().body(err);
    }

    let data = match fs::read_to_string(CACHE_PATH) {
        Ok(content) => content,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to read cache file"),
    };

    let items: Value = match serde_json::from_str(&data) {
        Ok(parsed) => parsed,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to parse JSON file"),
    };

    if let Some(item) = items.get(&name.into_inner()) {
        HttpResponse::Ok().json(item)
    } else {
        HttpResponse::NotFound().body("Item not found")
    }
}

