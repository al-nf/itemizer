/*
 * File: player.rs
 *
 * Copyright (c) 2025 Alan Fung
 *
 * Description: structs, implementations, and utility functions dealing with the local plyaer
 */ use serde::{Deserialize, Serialize};
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
    champion: String,
    level: u8,
    skill_points: [u8; 4],
    items: [u16; 6],
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
        champion: player.champ.clone(),
        level: player.level,
        skill_points: player.skill_points,
        items: player.items,
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
        Some(item) => { player.items[item] = 0;
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
            HttpResponse::BadRequest().body("Invalid input: needs to be either 'inc' or 'dec'")
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct UserStats {
    ability_power: f64,
    armor: f64,
    armor_penetration_and_lethality: (f64, f64),
    attack_damage: f64,
    attack_speed: f64,
    critical_strike_chance: f64,
    heal_and_shield_power: f64,
    health_and_regen: (f64, f64),
    lifesteal_and_omnivamp: (f64, f64),
    magic_penetration: (f64, f64),
    magic_resistance: f64,
    mana_and_regen: (f64, f64),
    movespeed: f64,
    ability_haste: f64,
    tenacity: f64,
}

pub async fn display_stats(player_data: web::Data<Mutex<Player>>) -> impl actix_web::Responder {
    let player = player_data.lock().await;

    let level = player.level as f64 - 1.0;

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

    let new_stats = UserStats {
        ability_power: merged.ability_power.flat + merged.ability_power.per_level * level * (0.7025 + 0.0175 * level),
        armor: merged.armor.flat + merged.armor.per_level * level * (0.7025 + 0.0175 * level),
        armor_penetration_and_lethality: (merged.armor_penetration.percent, merged.lethality.flat),
        attack_damage: merged.attack_damage.flat + merged.attack_damage.per_level * level * (0.7025 + 0.0175 * level),
        attack_speed: merged.attack_speed.flat + ((merged.attack_speed.percent / 100.0 + merged.attack_speed.per_level * level * (0.7025 + 0.0175 * level)) * merged.attack_speed.flat),
        critical_strike_chance: merged.critical_strike_chance.flat + merged.critical_strike_chance.per_level * level * (0.7025 + 0.0175 * level),
        heal_and_shield_power: merged.heal_and_shield_power.flat + merged.heal_and_shield_power.per_level * level * (0.7025 * 0.0175 * level),
        health_and_regen: (merged.health.flat + merged.health.per_level * level * (0.7025 * 0.0175 * level), merged.health_regen.flat + merged.health_regen.per_level * level * (0.7025 * 0.0175 * level)),
        lifesteal_and_omnivamp: (merged.lifesteal.percent + merged.lifesteal.per_level * level * (0.7025 * 0.0175 * level), merged.omnivamp.percent + merged.omnivamp.per_level * level * (0.7025 * 0.0175 * level)),
        magic_penetration: (merged.magic_penetration.percent, merged.magic_penetration.flat),
        magic_resistance: merged.magic_resistance.flat + merged.magic_resistance.per_level * level * (0.7025 * 0.0175 * level),
        mana_and_regen: (merged.mana.flat + merged.mana.per_level * level * (0.7025 * 0.0175 * level), merged.mana_regen.flat + merged.mana_regen.per_level * level * (0.7025 * 0.0175 * level)),
        movespeed: merged.movespeed.flat + merged.movespeed.per_level * level * (0.7025 * 0.0175 * level),
        ability_haste: merged.ability_haste.flat + merged.ability_haste.per_level * level * (0.7025 * 0.0175 * level),
        tenacity: merged.tenacity.percent + merged.tenacity.per_level * level * (0.7025 * 0.0175 * level)
    };
    
    HttpResponse::Ok().json(new_stats)
}
