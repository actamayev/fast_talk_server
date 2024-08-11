use actix_web::web;

use crate::handlers::auth_handlers::register;

pub fn register_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/register", web::post().to(register)),
    );
}
