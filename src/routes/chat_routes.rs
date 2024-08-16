use actix_web::web;
use crate::handlers::chat_handlers::create_chat::create_chat;
use crate::middleware::jwt_verify::JwtVerify;
use sea_orm::DatabaseConnection;

pub fn chat_routes(cfg: &mut web::ServiceConfig, db: web::Data<DatabaseConnection>) {
    let jwt_verify = JwtVerify::new(db.clone());

    cfg.service(
        web::scope("/chat")
            .wrap(jwt_verify) // Apply JwtVerify middleware to the entire /chat scope
            .service(
                web::resource("/create-private-chat")
                    .route(web::post().to(create_chat))
            )
            // Add more chat-related routes here, all of which will have JwtVerify middleware applied
    );
}
