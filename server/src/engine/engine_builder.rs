use std::sync::Arc;

use tokio::sync::mpsc::{channel, Receiver, Sender};
use webrtc_unreliable::Server as RtcServer;

use super::game::Game;
use super::network::network_manager::NetworkManager;
use super::network::service::{start_rtc_server, start_service};
use crate::engine::messaging::messages::*;
use crate::engine::resources::resource_manager::ResourceManager;

const CHANNEL_BUFFER_SIZE: usize = 512;

pub struct GameConfig {
    pub ticks_per_second: u8,
    pub service_address: String,
    pub public_address: String,
}

pub async fn build_engine(
    config: GameConfig,
    resource_manager: &Arc<ResourceManager>,
) -> (
    tokio::task::JoinHandle<()>,
    tokio::task::JoinHandle<()>,
    tokio::task::JoinHandle<()>,
) {
    //For Game --> NetworkManager
    let (out_reliable, reliable_out_queue) =
        channel::<(OutTarget, OutMessage)>(CHANNEL_BUFFER_SIZE);
    let (out_unreliable, unreliable_out_queue) =
        channel::<(OutTarget, OutMessage)>(CHANNEL_BUFFER_SIZE);

    //For NetworkManager --> Game
    let (game_sender, game_in) = channel::<GameMessage>(CHANNEL_BUFFER_SIZE);

    //For WS Clients --> NetworkManager
    let (ws_client_sender, ws_in) = channel::<WSClientMessage>(CHANNEL_BUFFER_SIZE);

    let rtc_server = start_rtc_server(config.service_address.clone(), config.public_address).await;
    let service = start_service(
        config.service_address,
        rtc_server.session_endpoint(),
        ws_client_sender,
    )
    .await;

    let network_handle = start_network_manager(
        ws_in,
        game_sender,
        reliable_out_queue,
        unreliable_out_queue,
        rtc_server,
    );

    let game_handle = start_game_thread(
        config.ticks_per_second,
        out_reliable,
        out_unreliable,
        game_in,
        resource_manager,
    );

    (game_handle, network_handle, service)
}

fn start_network_manager(
    ws_in: Receiver<WSClientMessage>,
    game_sender: Sender<GameMessage>,
    reliable_out_queue: Receiver<(OutTarget, OutMessage)>,
    unreliable_out_queue: Receiver<(OutTarget, OutMessage)>,
    rtc_server: RtcServer,
) -> tokio::task::JoinHandle<()> {
    let mut network_manager = NetworkManager::new(
        ws_in,
        game_sender,
        reliable_out_queue,
        unreliable_out_queue,
        rtc_server,
    );

    tokio::spawn(async move { network_manager.process().await })
}

fn start_game_thread(
    ticks_per_second: u8,
    out_reliable: Sender<(OutTarget, OutMessage)>,
    out_unreliable: Sender<(OutTarget, OutMessage)>,
    game_in: Receiver<GameMessage>,
    resource_manager: &Arc<ResourceManager>,
) -> tokio::task::JoinHandle<()> {
    let tick_time = 1. / ticks_per_second as f32;
    let inner = resource_manager.clone();

    tokio::spawn(async move {
        let map = inner.get_map("testMap").await.unwrap();
        let characters = Vec::new();

        Game::new(tick_time, out_reliable, out_unreliable, game_in)
            .init_game(map, characters)
            .start_game_loop()
            .await //todo this should return the result of the game
    })
}
