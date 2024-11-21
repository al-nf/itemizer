// src/main.rs

use actix_web::{web, App, HttpServer, Responder, HttpResponse};
mod download;
mod champion;
mod item;
mod runes;

async fn download_file() -> impl Responder {
    match download::download_and_extract().await {
        Ok(_) => HttpResponse::Ok().body("File downloaded and extracted successfully."),
        Err(e) => {
            // You can log the error or provide more details in the response
            HttpResponse::InternalServerError().body(format!("Failed to download and extract: {}", e))
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(download_file))  // Map root to download_file function
            .route("/champion", web::get().to(champion::get_champion_json))
            .route("/item", web::get().to(item::get_item_json))
            .route("/runes", web::get().to(runes::get_runes_json))
    })
    .bind("127.0.0.1:8080")?  // Bind the server to localhost on port 8080
    .run()
    .await
}
