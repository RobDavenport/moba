use ws::*;

mod network;
use network::client_manager::ClientManager;

use std::thread;

mod engine;
use engine::engine::Engine;

const IP_ADDRESS: &str = "127.0.0.1:8000";

fn main() {
    let game = Engine::new();
    let game_channel = game.request_channel();

    let ws_thread = thread::spawn(|| {
        let client_manager = ClientManager::new(game_channel);
        let websocket = WebSocket::<ClientManager>::new(client_manager).unwrap();
        println!("WS server listening at: {}", IP_ADDRESS);
        websocket.listen(IP_ADDRESS);
    });
  
    ws_thread.join().unwrap();
}
