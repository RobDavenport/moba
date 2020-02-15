use std::sync::{Arc, Mutex};

use webrtc_unreliable::Server as RtcServer;

use futures::{
    future::{Fuse, FusedFuture, FutureExt},
    pin_mut, select,
    stream::{FusedStream, FuturesUnordered, Stream, StreamExt},
};

use std::sync::mpsc::Receiver;

use crate::engine::messaging::message_listener::MessageListener;
use crate::engine::messaging::messages::{ OutMessage };

pub struct RtcServerRunner {
    //server_ptr: Arc<Mutex<RtcServer>>,
//rtc_server: RtcServer,
}

impl RtcServerRunner {
    // pub fn new(rtc_server: RtcServer) -> Self {
    //     Self {
    //         server_ptr: Arc::new(Mutex::new(rtc_server)),
    //     }
    //     // Self {
    //     //     rtc_server
    //     // }
    // }

    //TODO: Add way to read "out messages" from game_out_unreliable...
    pub async fn run_server(mut rtc_server: RtcServer) {
        let mut msg_buf = vec![0; 0x10000];
        tokio::spawn(async move {
            loop {
                match rtc_server.recv(&mut msg_buf).await {
                    Ok(msg) => println!("got rtc message"), //parse the message
                    Err(e) => println!("error: {}", e),
                }
            }
        }).await.unwrap();
    }

    // pub async fn send(&mut self, msg: String) {
    //     self.rtc_server.send().await
    // }
}
