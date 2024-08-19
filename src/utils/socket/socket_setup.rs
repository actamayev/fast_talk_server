use actix::prelude::*;
use actix_web::{web, HttpRequest, HttpResponse, Error};
use actix_web_actors::ws;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// Type alias for shared client map
pub type ClientMap = Arc<Mutex<HashMap<i32, Addr<MyWebSocket>>>>;

// Define a WebSocket message that can be sent to the clients
#[derive(Message)]
#[rtype(result = "()")]
struct WsMessage(String);

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
        ctx.text("You are connected");
    }

    fn stopped(&mut self, _: &mut Self::Context) {
        let mut clients = self.clients.lock().unwrap();
        clients.remove(&self.user_id);
    }
}

// Handle incoming WebSocket messages
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                // Here we could handle incoming messages if needed
                // For now, we simply echo the message back
                ctx.text(text);
            }
            _ => (),
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
    // Extract user ID from request (assumed to be part of the query or headers)
    let user_id = extract_user_id(&req);

    // Initialize the WebSocket connection
    let ws = MyWebSocket {
        user_id,
        clients: clients.get_ref().clone(),
    };
    ws::start(ws, &req, stream)
}

fn extract_user_id(req: &HttpRequest) -> i32 {
    // Extract the user ID from the request; for simplicity, we'll assume it's an integer
    // You might extract it from a query parameter, header, or token
    // Here, we're just hardcoding for illustration
    1 // Replace with actual logic to extract user_id
}
