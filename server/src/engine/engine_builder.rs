//use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

use tokio::sync::mpsc::{channel, Receiver, Sender};
use webrtc_unreliable::Server as RtcServer;
use webrtc_unreliable::SessionEndpoint;
use ws::*;

use hyper::{
    header::{self, HeaderValue},
    server::conn::AddrStream,
    service::{make_service_fn, service_fn},
    Body, Error, Method, Response, Server, StatusCode,
};

use super::game::Game;
use super::network::webrtc::rtc_client_manager::RtcClientManager;
use super::network::webrtc::rtc_server_runner::RtcServerRunner;
use super::network::ws::client_factory::ClientFactory;
use crate::engine::messaging::messages::{GameMessage, OutMessage};

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
    thread::JoinHandle<()>,
    tokio::task::JoinHandle<()>,
    tokio::task::JoinHandle<()>
) {
    let (game_sender_reliable, game_receiver_reliable) = channel::<GameMessage>(CHANNEL_BUFFER_SIZE);
    let (game_sender_unreliable, game_receiver_unreliable) = channel::<GameMessage>(CHANNEL_BUFFER_SIZE);

    let (out_sender_reliable, out_receiver_reliable) = channel::<OutMessage>(CHANNEL_BUFFER_SIZE);
    let (out_sender_unreliable, out_receiver_unreliable) = channel::<OutMessage>(CHANNEL_BUFFER_SIZE);

    let ws_thread = start_ws_server(
        config.ws_address,
        game_sender_reliable,
        out_receiver_reliable,
    );

    let rtc_server = start_rtc_server(config.rtc_listen, config.rtc_public).await;
    let listener = start_sdp_listener(config.sdp_address, rtc_server.session_endpoint()).await;
    let serv_handle = start_rtc_listener(rtc_server, game_sender_unreliable, out_receiver_unreliable).await;

    let game_thread = start_game_thread(
        config.ticks_per_second,
        out_sender_reliable,
        out_sender_unreliable,
        game_receiver_reliable,
        game_receiver_unreliable,
    );

    (ws_thread, game_thread, serv_handle, listener)
}

fn start_ws_server(
    address: String,
    game_sender: Sender<GameMessage>,
    out_receiver: Receiver<OutMessage>,
) -> thread::JoinHandle<()> {
    let client_factory = ClientFactory::new(game_sender, out_receiver, CHANNEL_BUFFER_SIZE);

    thread::spawn(|| {
        let ws_server = WebSocket::<ClientFactory>::new(client_factory, ).unwrap();
        println!("New WS server active at: {}", address);
        ws_server.listen(address).unwrap();
    })
}

fn start_game_thread(
    ticks_per_second: u8,
    out_sender_reliable: Sender<OutMessage>,
    out_sender_unreliable: Sender<OutMessage>,
    game_receiver_reliable: Receiver<GameMessage>,
    game_receiver_unreliable: Receiver<GameMessage>,
) -> thread::JoinHandle<()> {
    let tick_time = 1. / ticks_per_second as f32;

    thread::spawn(move || {
        Game::new(
            tick_time,
            out_sender_reliable,
            out_sender_unreliable,
            game_receiver_reliable,
            game_receiver_unreliable,
        )
        .start_loop();
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

async fn start_rtc_listener(
    rtc_server: RtcServer,
    game_sender_unreliable: Sender<GameMessage>,
    out_receiver_unreliable: Receiver<OutMessage>,
) -> tokio::task::JoinHandle<()> {
    RtcServerRunner::run_rtc_server(rtc_server, out_receiver_unreliable).await
}

async fn start_sdp_listener(sdp_addr: String, endpoint: SessionEndpoint) -> tokio::task::JoinHandle<()> {
    println!("start sdp listener");
    let make_svc = make_service_fn(move |addr_stream: &AddrStream| {
        let session_endpoint = endpoint.clone();
        let remote_addr = addr_stream.remote_addr();
        async move {
            Ok::<_, Error>(service_fn(move |req| {
                let mut session_endpoint = session_endpoint.clone();
                async move {
                    if req.uri().path() == "/sdp" && req.method() == Method::POST {
                        println!("WebRTC session request from {}", remote_addr);
                        match session_endpoint.http_session_request(req.into_body()).await {
                            Ok(mut resp) => {
                                resp.headers_mut().insert(
                                    header::ACCESS_CONTROL_ALLOW_ORIGIN,
                                    HeaderValue::from_static("*"),
                                );
                                Ok(resp.map(Body::from))
                            }
                            Err(err) => Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("error: {}", err))),
                        }
                    } else {
                        Response::builder()
                            .status(StatusCode::NOT_FOUND)
                            .body(Body::from("not found"))
                    }
                }
            }))
        }
    });

    tokio::spawn(async move {
        println!("Http SDP requests at: {}", sdp_addr);
        Server::bind(&sdp_addr.parse().unwrap())
            .serve(make_svc)
            .await
            .expect("HTTP session server has died");
    })
}
