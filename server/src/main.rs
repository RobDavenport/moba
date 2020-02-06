use ws::*;

mod network;
use network::ws_server::WsServer;

use std::thread;

mod engine;
use engine::engine::Engine;

const IP_ADDRESS: &str = "127.0.0.1:8000";

fn main() {
    let game = Engine::new();

    let ws_thread = thread::spawn(|| {
        let server = WsServer::new();
        let websocket = WebSocket::<WsServer>::new(server).unwrap();
        println!("WS server listening at: {}", IP_ADDRESS);
        websocket.listen(IP_ADDRESS);
    });
    
    ws_thread.join().unwrap();
}
