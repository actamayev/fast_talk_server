use actix_web::web;

use crate::handlers::auth_handlers::register::register;
use crate::handlers::auth_handlers::login::login;
use crate::middleware::auth_middleware::register_middleware::ValidateRegister;
// use crate::middleware::auth_middleware::login_middleware::ValidateLogin;

pub fn auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .wrap(ValidateRegister) // Apply the middleware for /register
            .route("/register", web::post().to(register))

            // .wrap(ValidateLogin) // Apply the middleware for /login
            .route("/login", web::post().to(login)),
    );
}
