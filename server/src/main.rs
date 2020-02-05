extern crate ws;
use ws::*;

mod network;
use network::server::Server;

fn main() {
    let websocket = WebSocket::<Server>::new(Server::new()).unwrap();
    websocket.listen("127.0.0.1:8000");
}
