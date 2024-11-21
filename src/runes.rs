// src/champion.rs
use actix_web::{HttpResponse, Responder};
use std::fs;
use std::path::Path;
use serde_json::Value;
use crate::download::get_latest_version;

pub async fn get_runes_json() -> impl Responder {
    match get_latest_version().await {
        Ok(latest_version) => {
            let file_path = format!("data/{}/data/en_US/runesReforged.json", latest_version);
            
            if !Path::new(&file_path).exists() {
                return HttpResponse::NotFound().body(format!("File not found: {}", file_path));
            }

            match fs::read_to_string(file_path) {
                Ok(contents) => {
                    match serde_json::from_str::<Value>(&contents) {
                        Ok(json) => HttpResponse::Ok().json(json), // Respond with the parsed JSON
                        Err(_) => HttpResponse::InternalServerError().body("Failed to parse JSON"),
                    }
                }
                Err(_) => HttpResponse::InternalServerError().body("Failed to read file"),
            }
        }
        Err(_) => HttpResponse::InternalServerError().body("Failed to get latest patch version"),
    }
}

