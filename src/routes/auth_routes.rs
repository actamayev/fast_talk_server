use actix_web::web;

use crate::handlers::auth_handlers::login::login;
use crate::handlers::auth_handlers::register::register;
use crate::middleware::auth_middleware::login_middleware::ValidateLogin;
use crate::middleware::auth_middleware::register_middleware::ValidateRegister;

pub fn auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            // Apply middleware for the login route
            .service(
                web::resource("/login")
                    .wrap(ValidateLogin)
                    .route(web::post().to(login))
            )
            // Apply middleware for the register route
            .service(
                web::resource("/register")
                    .wrap(ValidateRegister)
                    .route(web::post().to(register))
            )
    );
}
