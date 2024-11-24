// src/champion.rs
use actix_web::{web, HttpResponse, Responder};
use reqwest::Client;
use serde_json::Value;
use std::fs::{self, File};
use std::io::Write;
use std::sync::Mutex;
use std::path::Path;
use crate::stats::{Stat, Stats};


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
    if let Err(err) = ensure_cache().await {
        return HttpResponse::InternalServerError().body(err);
    }

    match fs::read_to_string(CACHE_PATH) {
        Ok(content) => HttpResponse::Ok()
            .content_type("application/json") // Explicitly set the content type
            .body(content),
        Err(_) => HttpResponse::InternalServerError().body("Failed to read cache file"),
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

pub async fn set_champion(
    stats: web::Data<Mutex<Stats>>,  
    champion_name: web::Path<String>,  
) -> impl Responder {
    let champion_name = champion_name.into_inner();
    let cache_path = "champs_cache.json";

    let data = match fs::read_to_string(cache_path) {
        Ok(content) => content,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to read cache file"),
    };

    let champs: Value = match serde_json::from_str(&data) {
        Ok(parsed) => parsed,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to parse cache file"),
    };

    if let Some(champion) = champs.get(&champion_name) {
        if let Some(base_stats) = champion.get("stats") {
            let mut stats = stats.lock().unwrap();
            match map_base_stats(&mut stats, base_stats) {
                Ok(()) => {
                    HttpResponse::Ok().body(format!("Champion {} stats updated successfully!", champion_name))
                }
                Err(err) => {
                    HttpResponse::InternalServerError().body(format!("Failed to map base stats: {}", err))
                }
            }
        } else {
            HttpResponse::NotFound().body("Champion base stats not found")
        }
    } else {
        HttpResponse::NotFound().body("Champion not found")
    }
}

fn map_base_stats(stats: &mut Stats, base_stats: &Value) -> Result<(), String> {
    let update_stat = |stat: &mut Stat, key: &str| {
        if let Some(value) = base_stats.get(key) {
            stat.flat = value.as_f64().unwrap_or(0.0); 
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
