use actix_web::web;
use sea_orm::DatabaseConnection;
use crate::handlers::chat_handlers::retrieve_chats_list::retrieve_chats_list;
use crate::middleware::chat_middleware::new_message_middleware::ValidateNewMessage;
use crate::middleware::jwt_verify::JwtVerify;

use crate::handlers::chat_handlers::create_chat::create_chat;
use crate::handlers::chat_handlers::send_message::send_message;

pub fn chat_routes(cfg: &mut web::ServiceConfig, db: web::Data<DatabaseConnection>) {
    let jwt_verify = JwtVerify::new(db.clone());

    cfg.service(
        web::scope("/chat")
            .wrap(jwt_verify) // Apply JwtVerify middleware to the entire /chat scope
            .service(
                web::resource("/create-chat/{friendId}") // Use friendId as a path parameter
                    .route(web::post().to(create_chat))
            )
            .service(
                web::resource("/send-message/{chatId}") // Use friendId as a path parameter
                    .wrap(ValidateNewMessage)
                    .route(web::post().to(send_message))
            )
            .service(
                web::resource("/retrieve-chats-list/{chatId}") // Use friendId as a path parameter
                    .wrap(ValidateNewMessage)
                    .route(web::post().to(retrieve_chats_list))
            )
    );
}
