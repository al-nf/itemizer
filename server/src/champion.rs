/*
 * File: champion.rs
 *
 * Copyright (c) 2025 Alan Fung
 *
 * Description: A collection of utility functions dealing with champions
 */
use actix_web::{web, HttpResponse, Responder};
use reqwest::Client;
use serde_json::Value;
use std::fs::{self, File};
use std::io::Write;
use tokio::sync::Mutex;
use std::path::Path;
use crate::stats::{Stat, Stats};
use crate::player::Player;


const CHAMP_URL: &str = "https://cdn.merakianalytics.com/riot/lol/resources/latest/en-US/champions.json";
const CHAMP_ICON_URL: &str = "https://cdn.communitydragon.org/latest/champion/";
const CHAMP_CACHE_PATH: &str = "public/champs_cache.json";
const CHAMP_ICON_CACHE_PATH: &str = "public/champ_icons";

/// Checks if champion data is cached. If not, creates the cache.
pub async fn ensure_champ_cache() -> Result<(), String> {
    if Path::new(CHAMP_CACHE_PATH).exists() {
        return Ok(());
    }

    let client = Client::new();
    let response = client
        .get(CHAMP_URL)
        .send()
        .await
        .map_err(|_| "Failed to fetch data from source".to_string())?;

    if response.status().is_success() {
        let body = response
            .text()
            .await
            .map_err(|_| "Failed to read response body".to_string())?;

        if let Err(_) = fs::create_dir_all(Path::new(CHAMP_CACHE_PATH).parent().unwrap()) {
            return Err("Failed to create cache directory".to_string());
        }

        let mut file = File::create(CHAMP_CACHE_PATH).map_err(|_| "Failed to create cache file".to_string())?;
        file.write_all(body.as_bytes())
            .map_err(|_| "Failed to write to cache file".to_string())?;

        Ok(())
    } else {
        Err(format!(
            "Failed to fetch data from source: HTTP {}",
            response.status()
        ))
    }
}

/// Deletes cached champion data and recreates the cache.
pub async fn update_champ_cache() -> Result<(), String> {
    if Path::new(CHAMP_CACHE_PATH).exists() {
        fs::remove_file(CHAMP_CACHE_PATH).map_err(|e| format!("Failed to delete champion cache: {}", e))?;
    }

    let client = Client::new();
    let response = client
        .get(CHAMP_URL)
        .send()
        .await
        .map_err(|_| "Failed to fetch data from source".to_string())?;

    if response.status().is_success() {
        let body = response
            .text()
            .await
            .map_err(|_| "Failed to read response body".to_string())?;

        if let Err(_) = fs::create_dir_all(Path::new(CHAMP_CACHE_PATH).parent().unwrap()) {
            return Err("Failed to create cache directory".to_string());
        }

        let mut file = File::create(CHAMP_CACHE_PATH).map_err(|_| "Failed to create cache file".to_string())?;
        file.write_all(body.as_bytes())
            .map_err(|_| "Failed to write to cache file".to_string())?;

        Ok(())
    } else {
        Err(format!(
            "Failed to fetch data from source: HTTP {}",
            response.status()
        ))
    }
}

/// Checks if champion icons are cached. If not, creates the cache.
pub async fn ensure_champ_icon_cache() -> Result<(), String> {
    if let Some(parent) = Path::new(CHAMP_ICON_CACHE_PATH).parent() {
        fs::create_dir_all(parent).map_err(|e| format!("Failed to create directory: {}", e))?;
    }

    let contains_files = || -> Result<bool, std::io::Error> {
        for entry in fs::read_dir(CHAMP_ICON_CACHE_PATH)? {
            let entry = entry?;
            if entry.file_type()?.is_file() {
                return Ok(true);
            }
        }
        Ok(false)
    };

    if !Path::new(CHAMP_ICON_CACHE_PATH).exists() {
        fs::create_dir(CHAMP_ICON_CACHE_PATH).map_err(|e| format!("Failed to create directory: {}", e))?;
    }

    if !contains_files().map_err(|e| e.to_string())? {
        println!("No files in champ icon cache directory");
        ensure_champ_cache().await.expect("Failed to ensure champion cache");
        let champs_json = fs::read_to_string(CHAMP_CACHE_PATH).expect("Unable to read champion cache");
        let json_value: Value = serde_json::from_str(&champs_json).expect("Invalid JSON format");

        let mut champs: Vec<String> = Vec::new();

        if let Value::Object(map) = json_value {
            for key in map.keys() {
                champs.push(key.to_string());
            }
        } else {
            println!("JSON not an object");
        }
        
        let client = Client::new();
        
        for champ in champs {
            for ability in ["p", "q", "w", "e", "r"] {
                let formatted_url = format!("{}{}/ability-icon/{}.png", CHAMP_ICON_URL, champ, ability);

                let response = client
                    .get(&formatted_url)
                    .send()
                    .await
                    .map_err(|e| format!("Failed to fetch icon: {}", e))?;

                if response.status().is_success() {
                    let file_name = format!("{}_{}.png", champ, ability);
                    let file_path = format!("{}/{}", CHAMP_ICON_CACHE_PATH, file_name);

                    if Path::new(&file_path).exists() {
                        println!("File already cached: {}", file_path);
                        continue;
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
                    println!("Failed to download file from: {}", formatted_url);
                }
            }
        }
    }
    Ok(())
}

/// Deletes cached champion icons and recreates the cache.
pub async fn update_champ_icon_cache() -> Result<(), String> {
    if let Some(parent) = Path::new(CHAMP_ICON_CACHE_PATH).parent() {
        fs::create_dir_all(parent).map_err(|e| format!("Failed to create directory: {}", e))?;
        if let Err(e) = fs::remove_dir_all(CHAMP_ICON_CACHE_PATH) {
            eprintln!("Failed to remove directory: {}", e);
        }
        fs::create_dir_all(CHAMP_ICON_CACHE_PATH).map_err(|e| format!("Failed to create directory: {}", e))?;
    }

    ensure_champ_cache().await.expect("Failed to ensure champion cache");
    let champs_json = fs::read_to_string(CHAMP_CACHE_PATH).expect("Unable to read champion cache");
    let json_value: Value = serde_json::from_str(&champs_json).expect("Invalid JSON format");

    let mut champs: Vec<String> = Vec::new();

    if let Value::Object(map) = json_value {
        for key in map.keys() {
            champs.push(key.to_string());
        }
    } else {
        println!("JSON not an object");
    }
    
    let client = Client::new();
    
    for champ in champs {
        for ability in ["p", "q", "w", "e", "r"] {
            let formatted_url = format!("{}{}/ability-icon/{}.png", CHAMP_ICON_URL, champ, ability);

            let response = client
                .get(&formatted_url)
                .send()
                .await
                .map_err(|e| format!("Failed to fetch icon: {}", e))?;

            if response.status().is_success() {
                let file_name = format!("{}_{}.png", champ, ability);
                let file_path = format!("{}/{}", CHAMP_ICON_CACHE_PATH, file_name);

                if Path::new(&file_path).exists() {
                    println!("File already cached: {}", file_path);
                    continue;
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
                println!("Failed to download file from: {}", formatted_url);
            }
        }
    }
    Ok(())
}

/// Retrieves all champion data.
pub async fn fetch_champs() -> impl Responder {
    if let Err(err) = ensure_champ_cache().await {
        return HttpResponse::InternalServerError().body(err);
    }

    match fs::read_to_string(CHAMP_CACHE_PATH) {
        Ok(content) => HttpResponse::Ok()
            .content_type("application/json") 
            .body(content),
        Err(_) => HttpResponse::InternalServerError().body("Failed to read cache file"),
    }
}

/// Retrieves a certain champion's data.
pub async fn get_champion(name: web::Path<String>) -> impl Responder {
    if let Err(err) = ensure_champ_cache().await {
        return HttpResponse::InternalServerError().body(err);
    }

    let data = match fs::read_to_string(CHAMP_CACHE_PATH) {
        Ok(content) => content,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to read cache file"),
    };

    let champs: Value = match serde_json::from_str(&data) {
        Ok(parsed) => parsed,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to parse JSON file"),
    };

    if let Some(champion) = champs.get(name.into_inner()) {
        HttpResponse::Ok().json(champion)
    } else {
        HttpResponse::NotFound().body("Champion not found")
    }
}

/// Provides quick access for a nested champion property. Might be useless.
pub async fn get_champion_property_nested(path: web::Path<(String, String)>) -> impl Responder {

    let data = match fs::read_to_string(CHAMP_CACHE_PATH) {
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

/// Updates the player with a given champion.
pub async fn set_champion(player_data: web::Data<Mutex<Player>>, champion_name: web::Path<String>) -> impl Responder {
    let champion_name = champion_name.into_inner();

    let mut player = player_data.lock().await;

    let data = match fs::read_to_string(CHAMP_CACHE_PATH) {
        Ok(content) => content,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to read cache file"),
    };

    let champs: Value = match serde_json::from_str(&data) {
        Ok(parsed) => parsed,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to parse cache file"),
    };

    if let Some(champion) = champs.get(&champion_name) {
        if let Some(base_stats) = champion.get("stats") {
            player.champ = champion_name.to_string();
            match map_base_stats(&mut player.base_stats, base_stats) {
                Ok(()) => {
                    HttpResponse::Ok().body(format!("Champion {} stats updated successfully!", champion_name))
                },
                Err(err) => HttpResponse::InternalServerError().body(format!("Failed to map base stats: {}", err)),
            }
        } else {
            HttpResponse::NotFound().body("Champion base stats not found")
        }
    } else {
        HttpResponse::NotFound().body("Champion not found")
    }
}

/// Helper function to map the player base stats
fn map_base_stats(stats: &mut Stats, base_stats: &Value) -> Result<(), String> {
    let update_stat = |stat: &mut Stat, key: &str| {
        if let Some(stat_data) = base_stats.get(key) {
            stat.flat = stat_data.get("flat").and_then(|v| v.as_f64()).unwrap_or(0.0);
            stat.percent = stat_data.get("percent").and_then(|v| v.as_f64()).unwrap_or(0.0);
            stat.per_level = stat_data.get("perLevel").and_then(|v| v.as_f64()).unwrap_or(0.0);
            stat.percent_base = stat_data.get("percentBase").and_then(|v| v.as_f64()).unwrap_or(0.0);
            stat.percent_bonus = stat_data.get("percentBonus").and_then(|v| v.as_f64()).unwrap_or(0.0);
        }
    };


    update_stat(&mut stats.armor, "armor");
    update_stat(&mut stats.attack_damage, "attackDamage");
    update_stat(&mut stats.attack_speed, "attackSpeed");
    update_stat(&mut stats.health, "health");
    update_stat(&mut stats.health_regen, "healthRegen");
    update_stat(&mut stats.magic_resistance, "magicResistance");
    update_stat(&mut stats.mana, "mana");
    update_stat(&mut stats.mana_regen, "manaRegen");
    update_stat(&mut stats.movespeed, "movespeed");

    Ok(())  
}

/// Fetches the champion associated with the local player.
pub async fn get_current_champion(player_data: web::Data<Mutex<Player>>) -> impl Responder {
    let player = player_data.lock().await;
    HttpResponse::Ok().body(player.champ.clone())
}
