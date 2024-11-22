/*
 * File: main.rs
 * Copyright: 2024, Alan Fung
 * Description: starts an app 
 */
use actix_web::{web, App, HttpServer, Responder, HttpResponse};
mod champion;
mod item;

async fn construction() -> impl Responder {
    HttpResponse::Ok().body("||| UNDER CONSTRUCTION |||")
}
/*
async fn get_champion_by_name(champion_name: web::Path<String>) -> impl Responder {
    // Fetch the champions data (this returns a serde_json::Value)
    let champions = champion::fetch_champs().await;

    // Check if the champions is a valid JSON array
    if let Some(champions_array) = champions.as_array() {
        // Try to find the champion with the matching 'name' field
        if let Some(champion) = champions_array.iter().find(|&champion| {
            // Access the 'name' field and check if it matches
            champion.get("name")
                .and_then(|name| name.as_str()) // Convert the value to a string slice
                .map_or(false, |name_str| name_str == champion_name)
        }) {
            // Return the champion as JSON if found
            return HttpResponse::Ok().json(champion);
        }
    }

    // If no champion was found, return a NotFound response
    HttpResponse::NotFound().body("Champion not found")
}
*/

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    champion::ensure_cache().await.expect("Failed to ensure champion cache");
    HttpServer::new(|| {
        App::new()
            .route("/champion", web::get().to(champion::fetch_champs))
            .route("/champion/{name}", web::get().to(champion::get_champion))
            .route("/fetch-champs", web::get().to(champion::fetch_champs))
            .route("/item", web::get().to(item::fetch_items))
            .route("/", web::get().to(construction))
    })
    .bind("127.0.0.1:8080")?  
    .run()
    .await
}
