// src/main.rs

use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use tokio;
mod download;

async fn get_link() -> impl Responder {
    match download::get_latest_patch().await {
        Ok(version) => HttpResponse::Ok().json(version),
        Err(_) => HttpResponse::InternalServerError().body("failed to fetch latest version"),
    }
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    /*
    tokio::spawn(async {
    if let Err(e) = download::download_and_extract().await {
        eprintln!("Error during download or extraction: {}", e);
    }
    });
    */
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_link)
        )
            
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

