use actix_web::web;
use sea_orm::DatabaseConnection;
use crate::handlers::chat_handlers::retrieve_chat_messages::retrieve_chat_messages;
use crate::handlers::chat_handlers::retrieve_chats_list::retrieve_chats_list;
use crate::middleware::chat_middleware::new_message_middleware::ValidateNewMessage;
use crate::middleware::jwt_verify::JwtVerify;

use crate::handlers::chat_handlers::create_chat::create_chat;
use crate::handlers::chat_handlers::send_message::send_message;

pub fn chat_routes(cfg: &mut web::ServiceConfig, db: web::Data<DatabaseConnection>) {
    let jwt_verify = JwtVerify::new(db.clone());

    cfg.service(
        web::scope("/chat")
            .service(
                web::resource("/create-chat/{friendId}")
                    .wrap(jwt_verify.clone())
                    .route(web::post().to(create_chat))
            )
            .service(
                web::resource("/send-message/{chatId}")
                    .wrap(jwt_verify.clone())
                    .wrap(ValidateNewMessage)
                    .route(web::post().to(send_message))
            )
            .service(
                web::resource("/retrieve-chats-list")
                    .wrap(jwt_verify.clone())
                    .route(web::get().to(retrieve_chats_list))
            )
            .service(
                web::resource("/retrieve-chat-messages/{chatId}")
                    .wrap(jwt_verify.clone())
                    .route(web::get().to(retrieve_chat_messages))
            )
    );
}

