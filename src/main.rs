mod routes;
mod middleware;
mod handlers;

use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(routes::auth_routes::auth_routes) // Configure all auth routes
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
