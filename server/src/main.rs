// src/main.rs
use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use actix_cors::Cors;
use actix_web::http::header;
use std::sync::Mutex;
mod champion;
mod item;
mod stats;

use crate::stats::Stats;

async fn get_stats_handler(stats: web::Data<Mutex<Stats>>) -> impl Responder {
    match stats.lock() {
        Ok(locked_stats) => HttpResponse::Ok().json(locked_stats.clone()), 
        Err(_) => HttpResponse::InternalServerError().body("Failed to lock stats"),
    }
}

pub async fn set_champion_handler(
    stats: web::Data<Mutex<Stats>>, 
    champion_name: web::Path<String>,
) -> impl Responder {
    let stats_clone = stats.clone();
    let response = champion::set_champion(stats_clone, champion_name).await;
    response
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    champion::ensure_cache().await.expect("Failed to ensure champion cache");
    let stats = web::Data::new(Mutex::new(Stats::new()));

    HttpServer::new(move || {
        App::new()
            .app_data(stats.clone())
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
            .route("/stats", web::get().to(get_stats_handler))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
