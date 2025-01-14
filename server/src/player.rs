/*
 * File: player.rs
 *
 * Copyright (c) 2025 Alan Fung
 *
 * Description: structs, implementations, and utility functions dealing with the local plyaer
 */
use serde::{Deserialize, Serialize};
use actix_web::{web, HttpResponse};
use tokio::sync::Mutex;

use crate::stats::Stats;
use crate::item::get_item_stats;

/// stores player information
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Player {
    /// level can range from 0 to 18
    pub level: u8,
    /// skill point cap should be enforced by the frontend
    pub skill_points: [u8; 4],
    /// base stats are provided by the champion
    pub base_stats: Stats,
    /// stats are provided by levels and items
    pub stats: Stats,
    /// items are stored as their ids
    pub items: [u16; 6],
    /// champion name is stored as a String for quick access
    pub champ: String
}

impl Default for Player {
    fn default() -> Self {
        Self::new()
    }
}

impl Player {
    pub fn new() -> Self {
        Player {
            level: 1,
            skill_points: [0; 4],
            base_stats: Stats::new(),
            stats: Stats::new(),
            items: [0; 6],
            champ: "none".to_string()
        }
    }

    pub fn create_player() -> Player {
        Player::new()
    }
}

#[derive(Serialize)]
struct PlayerStats {
    level: u8,
    skill_points: [u8; 4],
    stats: Stats
}

pub async fn get_player(player_data: web::Data<Mutex<Player>>) -> impl actix_web::Responder {
    let player = player_data.lock().await;

    let mut merged = Stats::add_stats(&player.base_stats, &player.stats);

    for item in player.items {
        if item != 0 {
            if let Some(item_stats) = &get_item_stats(item).await {
                merged = Stats::add_stats(&merged, item_stats);
            } else {
                println!("No item in slot {}", item);
            }
        }
    }
    let new_stats = PlayerStats {
        level: player.level,
        skill_points: player.skill_points,
        stats: merged
    };
    HttpResponse::Ok().json(new_stats)
}

pub async fn add_item(player_data: web::Data<Mutex<Player>>, path: web::Path<u16>) -> impl actix_web::Responder {
    let mut player = player_data.lock().await;
    let item_id = path.into_inner();

    match (0..6).find(|&i| player.items[i] == 0) {
        Some(item) => {
            player.items[item] = item_id;
            HttpResponse::Ok().body(format!("Successfully added item {} to player", item_id))
        }
        None => {
            println!("No inventory space.");
            HttpResponse::InternalServerError().body("Not enough space to add an item!")
        }
    }
}

pub async fn remove_last_item(player_data: web::Data<Mutex<Player>>) -> impl actix_web::Responder {
    let mut player = player_data.lock().await;

    match (0..6).rev().find(|&i| player.items[i] != 0) {
        Some(item) => {
            player.items[item] = 0;
            HttpResponse::Ok().body("Successfully removed last item from player")
        }
        None => {
            println!("No items to remove.");
            HttpResponse::InternalServerError().body("No items to remove!")
        }
    }
}

pub async fn set_item(player_data: web::Data<Mutex<Player>>, path: web::Path<(usize, u16)>) -> impl actix_web::Responder {
    let (item, item_id) = path.into_inner();
    if item > 5 {
        return HttpResponse::InternalServerError().body("Item index out of bounds");
    }
    let mut player = player_data.lock().await;

    player.items[item] = item_id;
    HttpResponse::Ok().body(format!("Successfully set item {} to id {}", item, item_id))
}

pub async fn change_skill_point(player_data: web::Data<Mutex<Player>>, path: web::Path<(usize, String)>) -> impl actix_web::Responder {
    let (ability, which_way) = path.into_inner();
    if ability > 3 || ability == 0 {
        return HttpResponse::InternalServerError().body("Ability index out of bounds");
    }

    let mut player = player_data.lock().await;

    match which_way.as_str() {
        "inc" => {
            player.skill_points[ability] += 1;
            HttpResponse::Ok().body(format!("Successfully increased the skill point of ability {}", ability))
        }
        "dec" => {
            player.skill_points[ability] -= 1;
            HttpResponse::Ok().body(format!("Successfully decreased the skill point of ability {}", ability))
        }
        _ => {
            HttpResponse::BadRequest().body("Invalid input: needs to be either 'plus' or 'minus'")
        }
    }
}
