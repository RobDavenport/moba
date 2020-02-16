#![recursion_limit = "1024"]

use futures::join;

mod engine;
use engine::engine_builder::*;

const WEB_RTC_LISTEN: &str = "192.168.1.150:8001";
const WEB_RTC_PUBLIC: &str = "192.168.1.150:8001";
const HYPER_API_ADDR: &str = "192.168.1.150:8001";

const WS_ADDRESS: &str = "0.0.0.0:8000";
const TICKS_PER_SECOND: u8 = 30;

#[tokio::main]
async fn main() {
    let game_config = GameConfig {
        ticks_per_second: TICKS_PER_SECOND,
        ws_address: WS_ADDRESS.to_string(),
        rtc_listen: WEB_RTC_LISTEN.to_string(),
        rtc_public: WEB_RTC_PUBLIC.to_string(),
        sdp_address: HYPER_API_ADDR.to_string(),
    };

    let (t1, game, rtc, sdp, ws) = build_engine(game_config).await;

    println!("engine running...");

    join!(game, rtc, sdp, ws);
    t1.join().unwrap();
}
