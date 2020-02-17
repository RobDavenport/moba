use webrtc_unreliable::MessageType;
use webrtc_unreliable::Server as RtcServer;

use tokio::sync::mpsc::Receiver;

use crate::engine::messaging::messages::OutMessage;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use futures::{future::FutureExt, select, stream::StreamExt};

pub struct RtcServerRunner {}

impl RtcServerRunner {
    pub fn run_rtc_server(
        mut rtc_server: RtcServer,
        mut out: Receiver<OutMessage>,
    ) -> tokio::task::JoinHandle<()> {
        tokio::spawn(async move {
            let mut msg_buf = vec![0; 0x10000];
            let mut targets = Vec::<SocketAddr>::new();
            println!("STARTED WEBRTC SERVER LOOP!");
            loop {
                select! {
                    msg = out.recv().fuse() => {
                        match msg {
                            Some(out_msg) => RtcServerRunner::handle_out_message(&mut rtc_server, out_msg, &mut targets).await,
                            None => panic!("received none!"),
                        };
                    },
                    rtc_msg = rtc_server.recv(&mut msg_buf).fuse() => {
                        match rtc_msg { //todo read the message, handle deserealizing
                            Ok(msg) => { //todo handle verified uuids
                                println!("got rtc message from: {}", msg.remote_addr);
                                let msg_text = &msg_buf[0..msg.message_len];
                                println!("{}", std::str::from_utf8(&msg_text).unwrap());
                                targets.push(msg.remote_addr);
                            },
                            Err(e) => println!("error: {}", e),
                        }
                    },
                };
            }
        })
    }

    async fn handle_out_message(
        rtc_server: &mut RtcServer,
        out_msg: OutMessage,
        mut targets: &Vec<SocketAddr>,
    ) {
        match out_msg {
            OutMessage::UpdateTick { .. } => {
                let output = serde_json::to_string(&out_msg).unwrap();
                for addr in targets {
                    rtc_server
                        .send(&output.as_bytes(), MessageType::Text, addr)
                        .await;
                }
            }
        }
    }
}
