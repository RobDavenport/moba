use std::sync::mpsc;
use std::sync::mpsc::*;

use super::game::Game;
use super::game_message::GameMessage;
use super::network::client_manager::ClientManager;
use super::network::out_message::OutMessage;

pub fn build_engine(ticks_per_second: u8) -> (Sender<GameMessage>, ClientManager) {
    let (game_sender, game_receiver) = mpsc::channel::<GameMessage>();
    let (out_sender, out_receiver) = mpsc::channel::<OutMessage>();
    let tick_time = 1. / ticks_per_second as f32;

    let client_manager = ClientManager::new(game_sender.clone(), out_receiver);

    std::thread::spawn(move || {
        Game::new(out_sender, game_receiver, tick_time).start_loop();
    });

    (game_sender, client_manager)
}

