use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};

use webrtc_unreliable::RecvError;
use webrtc_unreliable::RecvError::*;
use webrtc_unreliable::Server as RtcServer;

use crate::engine::messaging::message_listener::MessageListener;
use crate::engine::messaging::messages::{GameMessage, OutMessage};

const SLEEP_NANO_SECONDS: u64 = 1;

pub struct RtcClientManager {
    rtc_server: RtcServer,
    game_sender: Sender<GameMessage>,
    out_listener: MessageListener<OutMessage>,
}

impl RtcClientManager {
    pub fn new(
        rtc_server: RtcServer,
        game_sender: Sender<GameMessage>,
        out_receiver: Receiver<OutMessage>,
    ) -> Self {
        let out = Self {
            rtc_server: rtc_server,
            game_sender,
            out_listener: MessageListener::new(out_receiver),
        };

        // let mut message_buf = vec![0; 0x10000];

        // tokio::spawn(async move {
        //     println!("rtc client loop started");
        //     loop {
        //         match out.rtc_server.recv(&mut message_buf).await {
        //             Ok(received) => {
        //                 let msg = &message_buf[0..received.message_len];
        //                 println!("sender: {}", &received.remote_addr);
        //                 println!("got message: {:?}", msg);
        //             }
        //             Err(err) => {
        //                 println!("couldnt recv! {}", err);
        //             }
        //         }
        //     }
        // });

        out
    }

    pub fn start_looping(&mut self) {
        let sleep_duration = std::time::Duration::from_nanos(SLEEP_NANO_SECONDS);

        //std::thread::spawn(|| {
        println!("unreliable output message loop started");
        loop {
            if let Some(mut game_out_messages) = self.out_listener.check_messages() {
                for game_out_message in game_out_messages.drain(..) {
                    self.handle_game_out_message(game_out_message)
                }
            }

            std::thread::sleep(sleep_duration);
        }
        //});
    }

    fn handle_game_out_message(&self, msg: OutMessage) {
        match msg {
            OutMessage::UpdateTick { .. } => {
                let output = serde_json::to_string(&msg).unwrap();
                //todo broadcast the mess
            }
        }
    }
}
