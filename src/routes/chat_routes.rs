use actix_web::web;
use sea_orm::DatabaseConnection;
use crate::middleware::chat_middleware::new_message_middleware::ValidateNewMessage;
use crate::middleware::jwt_verify::JwtVerify;

use crate::handlers::chat_handlers::create_chat::create_chat;
use crate::handlers::chat_handlers::send_private_chat_message::send_private_chat_message;

pub fn chat_routes(cfg: &mut web::ServiceConfig, db: web::Data<DatabaseConnection>) {
    let jwt_verify = JwtVerify::new(db.clone());

    cfg.service(
        web::scope("/chat")
            .wrap(jwt_verify) // Apply JwtVerify middleware to the entire /chat scope
            .service(
                web::resource("/create-private-chat/{friendId}") // Use friendId as a path parameter
                    .route(web::post().to(create_chat))
            )
            .service(
                web::resource("/send-private-chat-message/{chatId}") // Use friendId as a path parameter
                    .wrap(ValidateNewMessage)
                    .route(web::post().to(send_private_chat_message))
            )
    );
}
