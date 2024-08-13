mod routes;
mod middleware;
mod handlers;

use actix_web::{App, HttpServer, web};
use handlers::health_handler::health_check;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(routes::auth_routes::auth_routes) // Configure all auth routes
            .route("/health", web::get().to(health_check)) // Add the /health route directly

    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
