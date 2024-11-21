/*
 * File: main.rs
 * Copyright: 2024, Alan Fung
 * Description: starts an app 
 */
use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde_json::{Value};
mod champion;
mod item;

async fn construction() -> impl Responder {
    HttpResponse::Ok().body("||| UNDER CONSTRUCTION |||")
}

/*
async fn get_champion_by_name(champion_name: web::Path<String>) -> impl Responder {
    let champions = champion::fetch_champs().await;

    // Try to find the champion with the matching 'name' field
    if let Some(champion) = champions.iter().find(|&champion| {
        if let Some(name) = champion.get("name") {
            name == champion_name
        } else {
            false
        }
    }) {
        HttpResponse::Ok().json(champion)
    } else {
        HttpResponse::NotFound().body("Champion not found")
    }
}
*/

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/champion", web::get().to(champion::fetch_champs))
            //.route("/champion/{champion-name}", web::get().to(get_champion_by_name))
            .route("/item", web::get().to(item::fetch_items))
            .route("/", web::get().to(construction))
    })
    .bind("127.0.0.1:8080")?  
    .run()
    .await
}
