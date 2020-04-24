#![recursion_limit = "1024"]
use std::env::var_os;
use std::sync::Arc;

use futures::join;

mod engine;
use engine::engine_builder::*;
use engine::resources::resource_manager::ResourceManager;

const WEB_SERVICE_ADDR: &str = "0.0.0.0";
const TICKS_PER_SECOND: u8 = 30;
const DEFAULT_PORT: &str = "8000";

#[tokio::main]
async fn main() {
    let port = match var_os("PORT") {
        Some(val) => val.into_string().unwrap(),
        None => String::from(DEFAULT_PORT),
    };

    println!("Use port: {}", &port);

    let ip = match my_internet_ip::get() {
        Ok(ip) => ip.to_string() + ":" + &port,
        Err(_e) => {
            println!("Couldn't get public IP. Local only.");
            WEB_SERVICE_ADDR.to_string() + ":" + &port
        }
    };

    let service_address = String::from(WEB_SERVICE_ADDR) + ":" + &port;

    println!("Server will operate at: {}", &ip);

    let game_config = GameConfig {
        ticks_per_second: TICKS_PER_SECOND,
        service_address: service_address,
        public_address: ip,
    };

    let resource_manager = Arc::new(ResourceManager::new());

    let (game, network, sdp) = build_engine(game_config, &resource_manager).await;

    println!("engine running...");

    join!(game, network, sdp,);
}
