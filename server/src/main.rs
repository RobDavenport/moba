use std::thread;

use ws::*;
use webrtc_unreliable::Server as RtcServer;

mod engine;
use engine::engine_builder::*;
use crate::engine::network::client_factory::ClientFactory;

const WEB_RTC_LISTEN: &str = "127.0.0.1:8001";
const WEB_RTC_PUBLIC: &str = "127.0.0.1:8001";

const IP_ADDRESS: &str = "192.168.1.150:8000";
const TICKS_PER_SECOND: u8 = 30;

#[tokio::main]
async fn main() {
    let client_factory = build_engine(TICKS_PER_SECOND);

    let ws_thread = thread::spawn(|| {
        let ws_server = WebSocket::<ClientFactory>::new(client_factory).unwrap();
        println!("WS server listening at: {}", IP_ADDRESS);
        ws_server.listen(IP_ADDRESS).unwrap();
    });

    ws_thread.join().unwrap();

    //TODO: WebRTC Server
    // let mut rtc_server = tokio::spawn(RtcServer::new(WEB_RTC_LISTEN.parse().unwrap(), WEB_RTC_PUBLIC.parse().unwrap())).await.unwrap().expect("rtc server boom");
    // let session_endpoint = rtc_server.session_endpoint();
}
