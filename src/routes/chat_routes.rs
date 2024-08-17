use actix_web::web;
use sea_orm::DatabaseConnection;
use crate::middleware::jwt_verify::JwtVerify;
use crate::middleware::chat_middleware::validate_friend_id::ValidateFriendId;
use crate::middleware::chat_middleware::check_if_chat_exists::CheckIfChatExists;

use crate::handlers::chat_handlers::create_chat::create_chat;

pub fn chat_routes(cfg: &mut web::ServiceConfig, db: web::Data<DatabaseConnection>) {
    let jwt_verify = JwtVerify::new(db.clone());
    let validate_friend_id = ValidateFriendId::new(db.clone());
    let check_if_chat_exists = CheckIfChatExists::new(db.clone());

    cfg.service(
        web::scope("/chat")
            .wrap(jwt_verify) // Apply JwtVerify middleware to the entire /chat scope
            .service(
                web::resource("/create-private-chat/{friendId}") // Use friendId as a path parameter
                    .wrap(validate_friend_id) // Apply ValidateFriendId middleware only to this route
                    .wrap(check_if_chat_exists)
                    .route(web::post().to(create_chat))
            )
    );
}
