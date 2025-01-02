use serde::{Deserialize, Serialize};
use actix_web::{web, HttpResponse};
use std::sync::Mutex;

use crate::stats::Stats;
use crate::item::get_item_stats;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Player {
    pub level: u8,
    pub skill_points: [u8; 4],
    pub base_stats: Stats,
    pub stats: Stats,
    pub items: [u8; 6], // Stores item IDs
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
    let player = match player_data.lock() {
        Ok(player) => player,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to lock player"),
    };

    let mut merged = Stats::add_stats(&player.base_stats, &player.stats);

    for item in player.items {
        if item != 0 {
            merged = Stats::add_stats(&merged, &get_item_stats(item).await.unwrap());
        }
    }
    let new_stats = PlayerStats {
        level: player.level,
        skill_points: player.skill_points,
        stats: merged
    };
    HttpResponse::Ok().json(new_stats)
}

pub async fn add_item(item: u8, player_data: web::Data<Mutex<Player>>) -> impl actix_web::Responder {
    let mut player = match player_data.lock() {
        Ok(player) => player,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to lock player"),
    };

    let find_first_slot = |player: &Player| -> Option<usize> {
        for i in 0..6 {
            if player.items[i] == 0 {
                return Some(i);
            }
        }
        None
    };

    match find_first_slot(&player) {
        Some(index) => {
            player.items[index] = item;
            HttpResponse::Ok().into()
        }
        None => {
            println!("No inventory space.");
            HttpResponse::Ok().into()
        }
    }
}

pub async fn remove_last_item(player_data: web::Data<Mutex<Player>>) -> impl actix_web::Responder {
    let mut player = match player_data.lock() {
        Ok(player) => player,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to lock player"),
    };

    let find_last_item = |player: &Player| -> Option<usize> {
        for i in (0..6).rev() {
            if player.items[i] != 0 {
                return Some(i);
            }
        }
        None
    };

    match find_last_item(&player) {
        Some(index) => {
            player.items[index] = 0;
            HttpResponse::Ok().into()
        }
        None => {
            println!("No items to remove.");
            HttpResponse::Ok().into()
        }
    }
}

pub async fn set_item(item: u8, player_data: web::Data<Mutex<Player>>, index: usize) -> impl actix_web::Responder {
    if index > 5 {
        return HttpResponse::InternalServerError().body("Item index out of bounds").into();
    }
    let mut player = match player_data.lock() {
        Ok(player) => player,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to lock player"),
    };

    player.items[index] = item;
    HttpResponse::Ok().into()
}

pub async fn change_skill_point(ability: usize, player_data: web::Data<Mutex<Player>>, decrease: bool) -> impl actix_web::Responder {
    if ability > 3 {
        return HttpResponse::InternalServerError().body("Ability index out of bounds").into();
    }

    let mut player = match player_data.lock() {
        Ok(player) => player,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to lock player"),
    };

    match decrease {
        false => {
            player.skill_points[ability] += 1;
            HttpResponse::Ok().into()
        }
        true => {
            player.skill_points[ability] -= 1;
            HttpResponse::Ok().into()
        }
    }
}
