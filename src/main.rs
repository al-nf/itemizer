// src/main.rs

use actix_web::{web, App, HttpServer, Responder, HttpResponse};
mod downloadlink;

async fn get_link() -> impl Responder {
    match downloadlink::get_latest_patch().await {
        Ok(version) => HttpResponse::Ok().json(version),
        Err(_) => HttpResponse::InternalServerError().body("failed to fetch latest version"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_link)
        )
            
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

