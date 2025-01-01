use serde::{Deserialize, Serialize};
use actix_web::{web, HttpResponse};
use std::sync::Mutex;

use crate::stats::Stats;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Player {
    pub level: u8,
    pub skill_points: [u8; 4],
    pub base_stats: Stats,
    pub stats: Stats,
    pub items: [u8; 6] // Stores item IDs
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
            items: [0; 6]
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

    let merged = Stats::add_stats(&player.base_stats, &player.stats);
    let new_stats = PlayerStats {
        level: player.level,
        skill_points: player.skill_points,
        stats: merged
    };
    HttpResponse::Ok().json(new_stats)
}

pub async fn add_item(item: u8, player_data: web::Data<Mutex<Player>>) -> impl actix_web::Responder {
    let player = match player_data.lock() {
        Ok(player) => player,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to lock player"),
    };

    let find_first_item = |player: &Player| -> u8 {
        for i in 0..6 {
            if player.items[i] == 0 {
                return i;
            }
        }
        7
    };
}
