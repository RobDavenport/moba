use std::cell::RefCell;
//use std::sync::mpsc::{Receiver, Sender};
use tokio::sync::mpsc::{Receiver, Sender};

use futures::{
    future::{Fuse, FusedFuture, FutureExt},
    pin_mut, select,
    stream::{FusedStream, FuturesUnordered, Stream, StreamExt},
};

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
        Self {
            rtc_server,
            game_sender,
            out_listener: MessageListener::new(out_receiver),
        }
    }

    pub fn start_looping(&mut self) {
        loop {
            if let Some(mut out_messages) = self.out_listener.check_messages() {
                for msg in out_messages.drain(..) {
                    self.handle_game_out_message(msg.clone());
                }
            }
            std::thread::sleep_ms(0);
        }
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
