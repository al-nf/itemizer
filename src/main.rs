/*
 * File: main.rs
 * Copyright: 2024, Alan Fung
 * Description: starts an app 
 */
use actix_web::{web, App, HttpServer, Responder, HttpResponse};
mod champion;
mod item;

async fn construction() -> impl Responder {
    HttpResponse::Ok().body("||| SITE UNDER CONSTRUCTION |||")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/champion", web::get().to(champion::fetch_champs))
            .route("/item", web::get().to(item::fetch_items))
            .route("/", web::get().to(construction))
    })
    .bind("127.0.0.1:8080")?  
    .run()
    .await
}
