use ws::*;

use std::thread;

mod engine;
use engine::engine_builder::*;

use crate::engine::network::client_manager::ClientManager;

const IP_ADDRESS: &str = "127.0.0.1:8000";
const TICKS_PER_SECOND: u8 = 30;

fn main() {
    let (game_channel, client_manager) = build_engine(TICKS_PER_SECOND);
    
    let ws_thread = thread::spawn(|| {
        let ws_server = WebSocket::<ClientManager>::new(client_manager).unwrap();
        println!("WS server listening at: {}", IP_ADDRESS);
        ws_server.listen(IP_ADDRESS);
    });

    ws_thread.join().unwrap();
}
