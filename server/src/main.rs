extern crate ws;
use ws::*;

mod network;
use network::ws_server::WsServer;

mod engine;

fn main() {
    let websocket = WebSocket::<WsServer>::new(WsServer::new()).unwrap();
    websocket.listen("127.0.0.1:8000");
}
