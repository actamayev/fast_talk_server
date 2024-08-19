use env_logger::Env;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use actix_web::{App, HttpServer, web};
use handlers::health_handler::health_check;
use utils::socket::socket_setup::{ws_index, ClientMap};

mod db;
mod utils;
mod types;
mod routes;
mod entities;
mod handlers;
mod middleware;
mod establish_connection;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logging
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();

    // Establish the database connection
    let db = establish_connection::establish_connection().await;
    let db_data = web::Data::new(db);

    // Create the shared client map
    let clients: ClientMap = Arc::new(Mutex::new(HashMap::new()));

    // Start the HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone()) // Pass the database connection to the app
            .app_data(web::Data::new(clients.clone())) // Pass the shared client map to the app
            .configure(routes::auth_routes::auth_routes) // Configure auth routes
            .configure(|cfg| routes::chat_routes::chat_routes(cfg, db_data.clone())) // Configure chat routes
            .route("/health", web::get().to(health_check)) // Add health check route
            .route("/ws/", web::get().to(ws_index)) // Add WebSocket route
    })
    .bind("127.0.0.1:8080")? // Bind to the localhost address
    .run()
    .await
}
