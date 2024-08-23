use serde_json::json;
use actix::prelude::*;
use actix_web_actors::ws;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use actix_web::{web, HttpRequest, HttpResponse, Error, error::InternalError};

use crate::utils::auth_helpers::jwt::decode_jwt;

// Type alias for shared client map
pub type ClientMap = Arc<Mutex<HashMap<i32, Addr<MyWebSocket>>>>;

// Define a WebSocket message that can be sent to the clients
#[derive(Message)]
#[rtype(result = "()")]
pub struct WsMessage(
   pub String
);

pub struct MyWebSocket {
    pub user_id: i32,
    pub clients: ClientMap,
}

impl Actor for MyWebSocket {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let addr = ctx.address();
        let mut clients = self.clients.lock().unwrap();
        clients.insert(self.user_id, addr);
    }

    fn stopped(&mut self, _: &mut Self::Context) {
        let mut clients = self.clients.lock().unwrap();
        clients.remove(&self.user_id);
    }
}

// Handle incoming WebSocket messages
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        if let Ok(ws::Message::Text(text)) = msg {
            ctx.text(text);
        }
    }
}

// Handle custom WsMessage
impl Handler<WsMessage> for MyWebSocket {
    type Result = ();

    fn handle(&mut self, msg: WsMessage, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

// WebSocket entry point
pub async fn ws_index(
    req: HttpRequest,
    stream: web::Payload,
    clients: web::Data<ClientMap>,
) -> Result<HttpResponse, Error> {
    // Extract user ID from request, propagating any errors
    let user_id = extract_user_id(&req).await?;

    // Initialize the WebSocket connection with the extracted user_id
    let ws = MyWebSocket {
        user_id,
        clients: clients.get_ref().clone(),
    };

    // Start the WebSocket connection
    ws::start(ws, &req, stream)
}

async fn extract_user_id(req: &HttpRequest) -> Result<i32, Error> {
    if let Some(query) = req.uri().query() {
        let params: HashMap<_, _> = url::form_urlencoded::parse(query.as_bytes()).into_owned().collect();
        if let Some(token) = params.get("token") {
            // Decode the JWT and get the user_id
            let claims = decode_jwt(token)?;
            let user_id = claims.sub.parse::<i32>().map_err(|_| {
                let response = HttpResponse::Unauthorized()
                    .json(json!({"message": "Invalid user ID in token"}));
                actix_web::Error::from(InternalError::from_response("", response))
            })?;
            return Ok(user_id);
        }
    }

    // If neither the header nor the query parameter has the token, return an error
    let response = HttpResponse::Unauthorized()
        .json(json!({"message": "Authorization header missing or invalid"}));
    Err(InternalError::from_response("", response).into())
}
