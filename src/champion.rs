/*
 * File: item.rs
 * Copyright: 2024, Alan Fung
 * Description: returns item.json
 */
use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use reqwest::Client;

pub async fn fetch_champs() -> impl Responder {
    let url = "http://cdn.merakianalytics.com/riot/lol/resources/latest/en-US/champions.json";
    
    // Create a new reqwest client
    let client = Client::new();

    // Send the GET request
    let response = client.get(url).send().await;

    match response {
        Ok(resp) => {
            if resp.status().is_success() {
                // Return the response body as the content directly
                let body = resp.text().await.unwrap_or_else(|_| String::from("Failed to read body"));
                HttpResponse::Ok().body(body)
            } else {
                // Handle failed HTTP request
                HttpResponse::InternalServerError().body("Failed to fetch data")
            }
        }
        Err(_) => {
            // Handle network error
            HttpResponse::InternalServerError().body("Network error while fetching data")
        }
    }
}
