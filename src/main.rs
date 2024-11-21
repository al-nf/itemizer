/*
 * File: main.rs
 * Copyright: 2024, Alan Fung
 * Description: starts an app 
 */
use actix_web::{web, App, HttpServer, Responder, HttpResponse};
mod download;
mod champion;
mod item;
mod runes;

async fn construction() -> impl Responder {
    HttpResponse::Ok().body("||| SITE UNDER CONSTRUCTION |||")
}

pub async fn download_handler() -> impl Responder {
    match download::check_and_update().await {
        Ok(_) => HttpResponse::Ok().body("Update completed successfully."),
        Err(err) => {
            eprintln!("Error during update: {}", err);
            HttpResponse::InternalServerError().body("Failed to update.")
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/download", web::get().to(download_handler)) 
            .route("/champion", web::get().to(champion::get_champion_json))
            .route("/item", web::get().to(item::get_item_json))
            .route("/runes", web::get().to(runes::get_runes_json))
            .route("/", web::get().to(construction))
    })
    .bind("127.0.0.1:8080")?  
    .run()
    .await
}
