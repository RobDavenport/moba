use std::sync::mpsc;

use super::game::Game;
use super::game_message::GameMessage;
use super::network::client_factory::ClientFactory;
use super::network::out_message::OutMessage;

pub fn build_engine(ticks_per_second: u8) -> ClientFactory {
    let (game_sender, game_receiver) = mpsc::channel::<GameMessage>();
    let (out_sender, out_receiver) = mpsc::channel::<OutMessage>();
    let tick_time = 1. / ticks_per_second as f32;

    let client_factory = ClientFactory::new(game_sender, out_receiver);

    std::thread::spawn(move || {
        Game::new(out_sender, game_receiver, tick_time).start_loop();
    });

    client_factory
}
