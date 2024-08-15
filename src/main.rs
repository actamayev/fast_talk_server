mod routes;
mod middleware;
mod handlers;
mod db;
mod entities;

use actix_web::{App, HttpServer, web};
use sea_orm::DatabaseConnection;
use std::sync::Arc;
use handlers::health_handler::health_check;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db: Arc<DatabaseConnection> = Arc::new(db::establish_connection().await);

    HttpServer::new(move || {
        App::new()
            .app_data(Arc::clone(&db)) // Make the DB connection available to handlers
            .configure(routes::auth_routes::auth_routes) // Configure all auth routes
            .route("/health", web::get().to(health_check)) // Add the /health route directly

    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
