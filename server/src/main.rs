// src/main.rs
use actix_web::{web, App, HttpServer, Responder};
use actix_cors::Cors;
use actix_web::http::header;
use std::sync::Mutex;
mod champion;
mod item;
mod stats;
mod player;

use crate::player::Player;

pub async fn set_champion_handler(
    player_data: web::Data<Mutex<Player>>,
    champion_name: web::Path<String>,
) -> impl Responder {
    let player_clone = player_data.clone();
    champion::set_champion(player_clone, champion_name).await
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    champion::ensure_champ_cache().await.expect("Failed to ensure champion cache");
    champion::ensure_champ_icon_cache().await.expect("Failed to ensure champion icon cache");
    item::ensure_item_cache().await.expect("Failed to ensure item cache");
    item::ensure_item_icon_cache().await.expect("Failed to ensure item icon cache");
    //champion::update_champ_cache().await.expect("Failed to update champion cache");
    //champion::update_champ_icon_cache().await.expect("Failed to update champion icon cache");
    //item::update_item_cache().await.expect("Failed to update item cache");
    //item::update_item_icon_cache().await.expect("Failed to update item icon cache");

    let player = web::Data::new(Mutex::new(Player::new()));

    HttpServer::new(move || {
        App::new()
            .app_data(player.clone())
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:5173")
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"]) 
                    .allowed_headers(vec![
                        header::CONTENT_TYPE,
                        header::ACCEPT,
                    ])
                    .max_age(3600),
            )
            .route("/champion", web::get().to(champion::fetch_champs))
            .route("/champion/{name}", web::get().to(champion::get_champion))
            .route("/champion/{name}/{property:.*}", web::get().to(champion::get_champion_property_nested))
            .route("/setchampion/{champion_name}", web::post().to(set_champion_handler))
            .route("/item", web::get().to(item::fetch_items))
            .route("/item/{name}", web::get().to(item::get_item))
            .route("/player", web::get().to(player::get_player))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

