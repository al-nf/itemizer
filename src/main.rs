// src/main.rs

use actix_web::{web, App, HttpServer, Responder, HttpResponse};
mod version; // Import the version module

async fn get_version() -> impl Responder {
    match version::get_latest_version().await {
        Ok(version) => HttpResponse::Ok().json(version),
        Err(_) => HttpResponse::InternalServerError().body("failed to fetch latest version"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_version)
        )
            
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

