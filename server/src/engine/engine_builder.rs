use std::thread;

use tokio::sync::mpsc::{channel, Receiver, Sender};
use webrtc_unreliable::Server as RtcServer;
use webrtc_unreliable::SessionEndpoint;
use ws::*;

use super::game::Game;
use super::network::network_manager::NetworkManager;
use super::network::webrtc::sdp_listener;
use super::network::ws::client_factory::ClientFactory;
use crate::engine::messaging::messages::*;

const CHANNEL_BUFFER_SIZE: usize = 256;

pub struct GameConfig {
    pub ticks_per_second: u8,
    pub ws_address: String,
    pub rtc_listen: String,
    pub rtc_public: String,
    pub sdp_address: String,
}

pub async fn build_engine(
    config: GameConfig,
) -> (
    thread::JoinHandle<()>,
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

    let ws_thread = start_ws_server(config.ws_address, ws_client_sender);

    let rtc_server = start_rtc_server(config.rtc_listen, config.rtc_public).await;
    let sdp_handle =
        sdp_listener::start_sdp_listener(config.sdp_address, rtc_server.session_endpoint()).await;

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
    );

    (ws_thread, game_handle, network_handle, sdp_handle)
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

fn start_ws_server(
    address: String,
    ws_client_sender: Sender<WSClientMessage>,
) -> thread::JoinHandle<()> {
    let ws_thread = thread::spawn(|| {
        let client_factory = ClientFactory::new(ws_client_sender);
        let ws_server = WebSocket::<ClientFactory>::new(client_factory).unwrap();
        println!("New WS server active at: {}", address);
        ws_server.listen(address).unwrap();
    });

    ws_thread
}

fn start_game_thread(
    ticks_per_second: u8,
    out_reliable: Sender<(OutTarget, OutMessage)>,
    out_unreliable: Sender<(OutTarget, OutMessage)>,
    game_in: Receiver<GameMessage>,
) -> tokio::task::JoinHandle<()> {
    let tick_time = 1. / ticks_per_second as f32;

    tokio::spawn(async move {
        Game::new(tick_time, out_reliable, out_unreliable, game_in)
            .start_game()
            .await //todo this should return the result of the game
    })
}

async fn start_rtc_server(listen_addr: String, public_addr: String) -> RtcServer {
    let rtc_server = tokio::spawn(RtcServer::new(
        listen_addr.parse().unwrap(),
        public_addr.parse().unwrap(),
    ))
    .await
    .unwrap()
    .expect("rtc server failed to start");

    rtc_server
}
