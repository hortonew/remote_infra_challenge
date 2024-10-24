use actix_web::{get, web, App, HttpServer, Responder, HttpResponse};
use std::env;
use dotenv::dotenv;
use serde_json::json;

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

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/index.html", web::get().to(index))
            .service(health)
    })
    .bind(&bind_address)?
    .run()
    .await
}
