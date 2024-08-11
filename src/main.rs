mod routes;
mod middleware;
mod handlers;

use actix_web::{App, HttpServer};
use crate::middleware::auth_middleware::ValidateRegister;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(ValidateRegister) // Apply the middleware
            .configure(routes::auth::register::register_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
