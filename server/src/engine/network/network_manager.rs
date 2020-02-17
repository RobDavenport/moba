use std::net::SocketAddr;

use tokio::sync::mpsc::{Receiver, Sender};
use webrtc_unreliable::Server as RtcServer;
use webrtc_unreliable::{MessageResult, MessageType};

use futures::{future::FutureExt, select, stream::StreamExt};

use super::client_data::ClientData;
use crate::engine::messaging::messages::{ClientMessage, GameMessage, OutMessage};

//todo: hook up game.rs to use this instead, and engine builders
//TODO: remove rtc_server_runner, remove client_manager_looper, merge
//into this file isntead
pub struct NetworkManager {
    clients: Vec<ClientData>,
    ws_in: Receiver<(u32, Vec<u8>)>,
    game_sender: Sender<GameMessage>,
    reliable_out_queue: Receiver<(u32, OutMessage)>,
    unreliable_out_queue: Receiver<(u32, OutMessage)>,
    rtc_server: RtcServer,
}

impl NetworkManager {
    pub fn new(
        ws_in: Receiver<(u32, Vec<u8>)>,
        game_sender: Sender<GameMessage>,
        reliable_out_queue: Receiver<(u32, OutMessage)>,
        unreliable_out_queue: Receiver<(u32, OutMessage)>,
        rtc_server: RtcServer,
    ) -> Self {
        Self {
            clients: Vec::new(),
            ws_in,
            game_sender,
            reliable_out_queue,
            unreliable_out_queue,
            rtc_server,
        }
    }

    pub async fn process(&mut self) {
        let mut msg_buf = vec![0; 0x10000];
        loop {
            select! {
                ws_result = self.ws_in.recv().fuse() => match in_result {
                    Some((id, in_bytes)) => handle_in_msg(id, in_bytes, &mut self.clients),
                    None => (),
                },
                reliable_result = self.reliable_out_queue.recv().fuse() => match reliable_result {
                    Some((target, out_reliable)) => handle_reliable_msg(target, out_reliable, &self.clients),
                    None => (),
                },
                unreliable_result = self.unreliable_out_queue.recv().fuse() => match unreliable_result {
                    Some((target, out_unreliable)) => handle_unreliable_msg(target, out_unreliable, &self.clients, &mut self.rtc_server),
                    None => (),
                },
                rtc_msg = self.rtc_server.recv(&mut msg_buf).fuse() => match rtc_msg {
                    Ok(msg) => handle_rtc_msg(msg, &msg_buf, &mut self.clients),
                    Err(e) => println!("rtc server: {}", e),
                },
            }
        }
    }
}

fn handle_in_msg(id: u32, in_bytes: Vec<u8>, clients: &mut Vec<ClientData>) {
    //unpack the message
    //process it
}

fn handle_reliable_msg(id: u32, out_msg: OutMessage, clients: &Vec<ClientData>) {
    //change to byte output
    match clients.iter().find(|client| client.id == id) {
        Some(client) => client
            .ws_client_out
            .send(serde_json::to_string(&out_msg).unwrap())
            .unwrap(), // change to msgpack
        None => (),
    }
}

fn handle_unreliable_msg(
    id: u32,
    out_msg: OutMessage,
    clients: &Vec<ClientData>,
    rtc_server: &mut RtcServer,
) {
    match clients
        .iter()
        .find(|client| client.id == id && client.socket_addr.is_some())
    {
        Some(client) => {
            rtc_server.send(
                serde_json::to_string(&out_msg).unwrap().as_bytes(),
                MessageType::Text,
                &client.socket_addr.unwrap(),
            );
        }
        None => (),
    }
}

fn handle_rtc_msg(msg: MessageResult, msg_buf: &Vec<u8>, clients: &mut Vec<ClientData>) {
    //todo read the message, handle deserealizing
    println!("got rtc message from: {}", msg.remote_addr);
    let msg_text = &msg_buf[0..msg.message_len];
    println!("{}", std::str::from_utf8(&msg_text).unwrap());
}
