use std::sync::mpsc;
use std::sync::mpsc::*;

use super::game::Game;
use super::game_message::GameMessage;
use super::network::client_manager::ClientManager;

pub fn build_engine(ticks_per_second: u8) -> (Sender<GameMessage>, ClientManager) {
    let (sender, receiver) = mpsc::channel::<GameMessage>();
    let tick_time = 1. / ticks_per_second as f32;

    let client_manager = ClientManager::new(sender.clone());

    std::thread::spawn(move || {
        Game::new(receiver, tick_time).start_loop();
    });

    (sender, client_manager)
}

