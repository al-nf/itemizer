/*
 * File: item.rs
 *
 * Copyright (c) 2025 Alan Fung
 *
 * Description: collection of utility functions dealing with items
 */
use actix_web::{web, HttpResponse, Responder};
use std::collections::HashMap;
use scraper::{Html, Selector};
use reqwest::Client;
use serde_json::Value;
use std::fs::{self, File};
use std::io::{Write, Read};
use std::path::Path;

use crate::stats::Stats;

const ITEM_URL: &str = "https://cdn.merakianalytics.com/riot/lol/resources/latest/en-US/items.json";
const ITEM_ICON_URL: &str = "https://raw.communitydragon.org/latest/plugins/rcp-be-lol-game-data/global/default/assets/items/icons2d";
const ITEM_CACHE_PATH: &str = "public/items_cache.json"; 
const ITEM_ICON_CACHE_PATH: &str = "public/item_icons"; 

/// Checks if item data is cached. If not, creates the cache.
pub async fn ensure_item_cache() -> Result<(), String> {
    if Path::new(ITEM_CACHE_PATH).exists() {
        return Ok(());
    }

    if let Some(parent) = Path::new(ITEM_CACHE_PATH).parent() {
        fs::create_dir_all(parent).map_err(|e| format!("Failed to create directory: {}", e))?;
    }

    let client = Client::new();
    let response = client
        .get(ITEM_URL)
        .send()
        .await
        .map_err(|e| format!("Failed to fetch data: {}", e))?;

    if response.status().is_success() {
        let body = response
            .text()
            .await
            .map_err(|e| format!("Failed to read response body: {}", e))?;

        let mut file = File::create(ITEM_CACHE_PATH).map_err(|e| format!("Failed to create cache file: {}", e))?;
        file.write_all(body.as_bytes())
            .map_err(|e| format!("Failed to write to cache file: {}", e))?;

        Ok(())
    } else {
        Err(format!("HTTP Error: {}", response.status()))
    }
}

/// Deletes cached item data and recreates the cache.
pub async fn update_item_cache() -> Result<(), String> {
    if Path::new(ITEM_CACHE_PATH).exists() {
        fs::remove_file(ITEM_CACHE_PATH).map_err(|e| format!("Failed to delete item cache: {}", e))?;
    }

    if let Some(parent) = Path::new(ITEM_CACHE_PATH).parent() {
        fs::create_dir_all(parent).map_err(|e| format!("Failed to create directory: {}", e))?;
    }

    let client = Client::new();
    let response = client
        .get(ITEM_URL)
        .send()
        .await
        .map_err(|e| format!("Failed to fetch data: {}", e))?;

    if response.status().is_success() {
        let body = response
            .text()
            .await
            .map_err(|e| format!("Failed to read response body: {}", e))?;

        let mut file = File::create(ITEM_CACHE_PATH).map_err(|e| format!("Failed to create cache file: {}", e))?;
        file.write_all(body.as_bytes())
            .map_err(|e| format!("Failed to write to cache file: {}", e))?;

        Ok(())
    } else {
        Err(format!("HTTP Error: {}", response.status()))
    }
}

/// Checks if item icons are cached. If not, creates the cache.
pub async fn ensure_item_icon_cache() -> Result<(), String> {
    if let Some(parent) = Path::new(ITEM_ICON_CACHE_PATH).parent() {
        fs::create_dir_all(parent).map_err(|e| format!("Failed to create directory: {}", e))?;
    }

    let contains_files = || -> Result<bool, std::io::Error> {
        for thing in fs::read_dir(ITEM_ICON_CACHE_PATH)? {
            let entry = thing?;
            if entry.file_type()?.is_file() {
                return Ok(true);
            }
        }
        Ok(false)
    };

    if !Path::new(ITEM_ICON_CACHE_PATH).exists() {
        fs::create_dir(ITEM_ICON_CACHE_PATH).map_err(|e| format!("Failed to create directory: {}", e))?;
    }
    
    if !contains_files().map_err(|e| e.to_string())? {
        println!("No files in item icon cache directory");
        let client = Client::new();

        let response = client
            .get(ITEM_ICON_URL)
            .send()
            .await
            .map_err(|e| format!("Failed to fetch data: {}", e))?;

        if response.status().is_success() {
            let body = response
                .text()
                .await
                .map_err(|e| format!("Failed to read response body: {}", e))?;

            let document = Html::parse_document(&body);
            let selector = Selector::parse("body > main > table > tbody > tr > td.link > a")
                .map_err(|_| "Failed to parse selector".to_string())?;
            let png_files: Vec<String> = document
                .select(&selector)
                .filter_map(|element| {
                    let href = element.value().attr("href")?;
                    if href.ends_with(".png") {
                        Some(href.to_string())
                    } else {
                        None
                    }
                })
                .collect();

            println!("Detected PNG files:");
            for file in &png_files {
                println!("{}", file);
            }

            for file in png_files {
                let file_url = format!("{}/{}", ITEM_ICON_URL, file);

                //println!("Downloading file: {}", file_url);

                let file_name = file.split('/').last().unwrap_or("unknown.png");
                let file_path = format!("{}/{}", ITEM_ICON_CACHE_PATH, file_name);

                if Path::new(&file_path).exists() {
                    println!("File already cached: {}", file_path);
                    continue;
                }

                let response = client
                    .get(&file_url)
                    .send()
                    .await
                    .map_err(|e| format!("Failed to fetch file: {}", e))?;

                if response.status().is_success() {
                    if let Some(parent) = Path::new(&file_path).parent() {
                        fs::create_dir_all(parent)
                            .map_err(|e| format!("Failed to create directory: {}", e))?;
                    }

                    let content = response
                        .bytes()
                        .await
                        .map_err(|e| format!("Failed to read file content: {}", e))?;

                    let mut output_file = File::create(&file_path)
                        .map_err(|e| format!("Failed to create file: {}", e))?;

                    output_file
                        .write_all(&content)
                        .map_err(|e| format!("Failed to write file: {}", e))?;

                    println!("Cached file: {}", file_path);
                } else {
                    println!("Failed to download file: {}", file_url);
                }
            }

            Ok(())
        } else {
            Err(format!("HTTP Error: {}", response.status()))
        }
    } else {
        Ok(())
    }
}

/// Deletes cached item icons and recreates the cache.
pub async fn update_item_icon_cache() -> Result<(), String> {
    if let Some(parent) = Path::new(ITEM_ICON_CACHE_PATH).parent() {
        fs::create_dir_all(parent).map_err(|e| format!("Failed to create directory: {}", e))?;
        if let Err(e) = fs::remove_dir_all(ITEM_ICON_CACHE_PATH) {
            eprintln!("Failed to remove directory: {}", e);
        }
        fs::create_dir_all(ITEM_ICON_CACHE_PATH).map_err(|e| format!("Failed to create directory: {}", e))?;
    }

    let client = Client::new();

    let response = client
        .get(ITEM_ICON_URL)
        .send()
        .await
        .map_err(|e| format!("Failed to fetch data: {}", e))?;

    if response.status().is_success() {
        let body = response
            .text()
            .await
            .map_err(|e| format!("Failed to read response body: {}", e))?;

        let document = Html::parse_document(&body);
        let selector = Selector::parse("body > main > table > tbody > tr > td.link > a")
            .map_err(|_| "Failed to parse selector".to_string())?;
        let png_files: Vec<String> = document
            .select(&selector)
            .filter_map(|element| {
                let href = element.value().attr("href")?;
                if href.ends_with(".png") {
                    Some(href.to_string())
                } else {
                    None
                }
            })
            .collect();

        /*
        println!("Detected PNG files:");
        for file in &png_files {
            println!("{}", file);
        }
        */

        for file in png_files {
            let file_url = format!("{}/{}", ITEM_ICON_URL, file);

            println!("Downloading file: {}", file_url);

            let file_name = file.split('/').last().unwrap_or("unknown.png");
            let file_path = format!("{}/{}", ITEM_ICON_CACHE_PATH, file_name);

            if Path::new(&file_path).exists() {
                println!("File already cached: {}", file_path);
                continue;
            }

            let response = client
                .get(&file_url)
                .send()
                .await
                .map_err(|e| format!("Failed to fetch file: {}", e))?;

            if response.status().is_success() {
                if let Some(parent) = Path::new(&file_path).parent() {
                    fs::create_dir_all(parent)
                        .map_err(|e| format!("Failed to create directory: {}", e))?;
                }

                let content = response
                    .bytes()
                    .await
                    .map_err(|e| format!("Failed to read file content: {}", e))?;

                let mut output_file = File::create(&file_path)
                    .map_err(|e| format!("Failed to create file: {}", e))?;

                output_file
                    .write_all(&content)
                    .map_err(|e| format!("Failed to write file: {}", e))?;

                println!("Cached file: {}", file_path);
            } else {
                println!("Failed to download file: {}", file_url);
            }
        }
        Ok(())

    } else {
        Err(format!("HTTP Error: {}", response.status()))
    }
}

/// Retrieves all item data.
pub async fn fetch_items() -> impl Responder {
    if let Err(err) = ensure_item_cache().await {
        return HttpResponse::InternalServerError().body(err);
    }

    match fs::read_to_string(ITEM_CACHE_PATH) {
        Ok(content) => HttpResponse::Ok()
            .content_type("application/json") 
            .body(content),
        Err(_) => HttpResponse::InternalServerError().body("Failed to read cache file"),
    }
}

/// Retrieves a certain item's data.
pub async fn get_item(name: web::Path<String>) -> impl Responder {
    if let Err(err) = ensure_item_cache().await {
        return HttpResponse::InternalServerError().body(err);
    }

    let data = match fs::read_to_string(ITEM_CACHE_PATH) {
        Ok(content) => content,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to read cache file"),
    };

    let items: Value = match serde_json::from_str(&data) {
        Ok(parsed) => parsed,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to parse JSON file"),
    };

    if let Some(items_map) = items.as_object() {
        for (_id, item) in items_map {
            if let Some(item_name) = item.get("name").and_then(|n| n.as_str()) {
                if item_name.eq_ignore_ascii_case(&name) {
                    return HttpResponse::Ok().json(item);
                }
            }
        }
    }

    HttpResponse::NotFound().body("Item not found")
}

pub async fn get_item_id(name: web::Path<String>) -> impl Responder {
    if let Err(err) = ensure_item_cache().await {
        return HttpResponse::InternalServerError().body(err);
    }

    let data = match fs::read_to_string(ITEM_CACHE_PATH) {
        Ok(content) => content,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to read cache file"),
    };

    let items: serde_json::Value = match serde_json::from_str(&data) {
        Ok(parsed) => parsed,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to parse JSON file"),
    };

    if let Some(items_map) = items.as_object() {
        for (key, item) in items_map {
            if let Some(item_name) = item.get("name").and_then(|n| n.as_str()) {
                if item_name.eq_ignore_ascii_case(&name) {
                    return HttpResponse::Ok().body(key.clone());
                }
            }
        }
    }

    HttpResponse::NotFound().body("Item not found")
}


/// Retrieves the stats of a given item.
pub async fn get_item_stats(id: u16) -> Option<Stats> {
    let mut file = File::open(ITEM_CACHE_PATH).expect("Unable to open file");
    let mut data = String::new();
    ensure_item_cache().await.expect("Failed to ensure item cache");
    file.read_to_string(&mut data).expect("Unable to read file");

    let items: HashMap<String, HashMap<String, serde_json::Value>> = 
        serde_json::from_str(&data).expect("JSON poorly formatted");

    items.values()
        .find(|item| item.get("id").and_then(|v| v.as_u64()).map(|v| v as u16) == Some(id))
        .and_then(|item| {
            let mut stats: Stats = serde_json::from_value(item.get("stats")?.clone()).ok()?;

            // stupid attack speed workaround
            let attack_speed_stat = &mut stats.attack_speed;
            if attack_speed_stat.flat != 0.0 { 
                attack_speed_stat.percent += attack_speed_stat.flat; 
                attack_speed_stat.flat = 0.0; 
            }

            Some(stats)
        })
}
