use std::thread;

use tokio::sync::mpsc::{channel, Receiver, Sender};
use webrtc_unreliable::Server as RtcServer;
use webrtc_unreliable::SessionEndpoint;
use ws::*;

use super::game::Game;
use super::network::webrtc::sdp_listener;
use super::network::webrtc::rtc_server_runner::RtcServerRunner;
use super::network::ws::client_factory::ClientFactory;
use super::network::ws::client_manager_looper::ClientManagerLooper;
use crate::engine::messaging::messages::*;

const CHANNEL_BUFFER_SIZE: usize = 100;

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
    tokio::task::JoinHandle<()>,
) {
    let (game_sender_reliable, game_receiver_reliable) =
        channel::<GameMessage>(CHANNEL_BUFFER_SIZE);
    let (game_sender_unreliable, game_receiver_unreliable) =
        channel::<GameMessage>(CHANNEL_BUFFER_SIZE);

    let (out_sender_reliable, out_receiver_reliable) = channel::<OutMessage>(CHANNEL_BUFFER_SIZE);
    let (out_sender_unreliable, out_receiver_unreliable) =
        channel::<OutMessage>(CHANNEL_BUFFER_SIZE);

    let (ws_thread, ws_looper) = start_ws_server(
        config.ws_address,
        game_sender_reliable,
        out_receiver_reliable,
    );

    let rtc_server = start_rtc_server(config.rtc_listen, config.rtc_public).await;
    let listener = sdp_listener::start_sdp_listener(config.sdp_address, rtc_server.session_endpoint()).await;
    let serv_handle =
        start_rtc_listener(rtc_server, game_sender_unreliable, out_receiver_unreliable);

    let game_handle = start_game_thread(
        config.ticks_per_second,
        out_sender_reliable,
        out_sender_unreliable,
        game_receiver_reliable,
        game_receiver_unreliable,
    );

    (ws_thread, game_handle, serv_handle, listener, ws_looper)
}

fn start_ws_server(
    address: String,
    game_sender: Sender<GameMessage>,
    out_receiver: Receiver<OutMessage>,
) -> (thread::JoinHandle<()>, tokio::task::JoinHandle<()>) {
    let (client_sender, client_receiver) = channel::<ClientMessage>(CHANNEL_BUFFER_SIZE);

    let task_handle =
        ClientManagerLooper::spawn_client_looper(client_receiver, out_receiver, game_sender);

    let ws_thread = thread::spawn(|| {
        let client_factory = ClientFactory::new(client_sender);
        let ws_server = WebSocket::<ClientFactory>::new(client_factory).unwrap();
        println!("New WS server active at: {}", address);
        ws_server.listen(address).unwrap();
    });

    (ws_thread, task_handle)
}

fn start_game_thread(
    ticks_per_second: u8,
    out_sender_reliable: Sender<OutMessage>,
    out_sender_unreliable: Sender<OutMessage>,
    game_receiver_reliable: Receiver<GameMessage>,
    game_receiver_unreliable: Receiver<GameMessage>,
) -> tokio::task::JoinHandle<()> {
    let tick_time = 1. / ticks_per_second as f32;

    tokio::spawn(async move {
        Game::new(
            tick_time,
            out_sender_reliable,
            out_sender_unreliable,
            game_receiver_reliable,
            game_receiver_unreliable,
        )
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

fn start_rtc_listener(
    rtc_server: RtcServer,
    game_sender_unreliable: Sender<GameMessage>,
    out_receiver_unreliable: Receiver<OutMessage>,
) -> tokio::task::JoinHandle<()> {
    RtcServerRunner::run_rtc_server(rtc_server, out_receiver_unreliable)
}
