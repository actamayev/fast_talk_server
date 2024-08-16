mod db;
mod utils;
mod types;
mod routes;
mod entities;
mod handlers;
mod middleware;
mod establish_connection;

use actix_web::{App, HttpServer, web};
use handlers::health_handler::health_check;
use env_logger::Env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();

    let db = establish_connection::establish_connection().await;
    let db_data = web::Data::new(db);

    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone()) // Clone the web::Data and pass it to the app
            .configure(routes::auth_routes::auth_routes) // Configure all auth routes
            .route("/health", web::get().to(health_check)) // Add the /health route directly
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
