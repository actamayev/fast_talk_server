use actix_web::web;
use sea_orm::DatabaseConnection;

use crate::middleware::jwt_verify::JwtVerify;
use crate::handlers::auth_handlers::login::login;
use crate::handlers::auth_handlers::register::register;
use crate::middleware::auth_middleware::login_middleware::ValidateLogin;
use crate::middleware::auth_middleware::register_middleware::ValidateRegister;
use crate::handlers::auth_handlers::retrieve_personal_info::retrieve_personal_info;

pub fn auth_routes(cfg: &mut web::ServiceConfig, db: web::Data<DatabaseConnection>) {
    let jwt_verify = JwtVerify::new(db.clone());

    cfg.service(
        web::scope("/auth")
            .service(
                web::resource("/login")
                    .wrap(ValidateLogin)
                    .route(web::post().to(login))
            )
            .service(
                web::resource("/register")
                    .wrap(ValidateRegister)
                    .route(web::post().to(register))
            )
            .service(
                web::resource("/retrieve-personal-info")
                    .wrap(jwt_verify.clone())
                    .route(web::get().to(retrieve_personal_info))
            )
    );
}
