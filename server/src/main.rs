/*
 * File: main.rs
 * Copyright: 2024, Alan Fung
 * Description: starts an app 
 */
use actix_web::{web, App, HttpServer, Responder, HttpResponse};
mod champion;
mod item;
mod stats;

async fn construction() -> impl Responder {
    HttpResponse::Ok().body("||| UNDER CONSTRUCTION |||")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    champion::ensure_cache().await.expect("Failed to ensure champion cache");
    let stats = Stats::new();
    HttpServer::new(|| {
        App::new()
            .route("/champion", web::get().to(champion::fetch_champs))
            .route("/champion/{name}", web::get().to(champion::get_champion))
            .route("/champion/{name}/{property:.*}", web::get().to(champion::get_champion_property_nested))
            .route("item/{name}", web::get().to(item::get_item))
            .route("/item", web::get().to(item::fetch_items))
            .route("/", web::get().to(construction))
            .route("/stats", web::get().to(stats
    })
    .bind("127.0.0.1:8080")?  
    .run()
    .await
}
