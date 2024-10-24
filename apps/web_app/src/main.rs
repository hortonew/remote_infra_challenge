use actix_web::{get, App, HttpServer, Responder, HttpResponse};
use std::env;
use dotenv::dotenv;
use serde_json::json;

#[get("/")]
async fn index() -> impl Responder {
    // Use the INDEX_RESPONSE environment variable if it exists, otherwise use "a"
    let response = env::var("INDEX_RESPONSE").unwrap_or_else(|_| "a".to_string());
    HttpResponse::Ok().body(response)
}

#[get("/health")]
async fn health() -> impl Responder {
    let response = env::var("INDEX_RESPONSE").unwrap_or_else(|_| "a".to_string());
    let health_response = json!({
        "status": "ok",
        "server": response
    });
    HttpResponse::Ok().json(health_response)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load the .env file
    dotenv().ok();
    let port = env::var("PORT").unwrap_or_else(|_| "80".to_string());
    let bind_address = format!("0.0.0.0:{}", port);

    // Start the web server on port 80
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(health)
    })
    .bind(&bind_address)?
    .run()
    .await
}
