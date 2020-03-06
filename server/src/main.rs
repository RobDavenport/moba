#![recursion_limit = "1024"]

use futures::join;

mod engine;
use engine::engine_builder::*;

const WEB_RTC_LISTEN: &str = "0.0.0.0:8000";
const LOCAL_IP: &str = "127.0.0.1:8000";
const WEB_SERVICE_ADDR: &str = "0.0.0.0:8001";

const WS_ADDRESS: &str = "0.0.0.0:8000";
const TICKS_PER_SECOND: u8 = 30;

#[tokio::main]
async fn main() {
    let ip = match my_internet_ip::get() {
        Ok(ip) => ip.to_string() + ":8000",
        Err(e) => {
            println!("Couldn't get public IP. Local only.");
            LOCAL_IP.to_string()
        }
    };

    println!("{}", ip);

    let game_config = GameConfig {
        ticks_per_second: TICKS_PER_SECOND,
        ws_address: WS_ADDRESS.to_string(),
        rtc_listen: WEB_RTC_LISTEN.to_string(),
        rtc_public: ip,
        sdp_address: WEB_SERVICE_ADDR.to_string(),
    };

    let (t1, game, network, sdp) = build_engine(game_config).await;

    println!("engine running...");

    join!(game, network, sdp,);
    t1.join().unwrap();
}
