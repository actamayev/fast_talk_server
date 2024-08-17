use actix_web::web;
use sea_orm::DatabaseConnection;
use crate::middleware::jwt_verify::JwtVerify;

use crate::handlers::chat_handlers::create_chat::create_chat;

pub fn chat_routes(cfg: &mut web::ServiceConfig, db: web::Data<DatabaseConnection>) {
    let jwt_verify = JwtVerify::new(db.clone());

    cfg.service(
        web::scope("/chat")
            .wrap(jwt_verify) // Apply JwtVerify middleware to the entire /chat scope
            .service(
                web::resource("/create-private-chat/{friendId}") // Use friendId as a path parameter
                    .route(web::post().to(create_chat))
            )
    );
}
