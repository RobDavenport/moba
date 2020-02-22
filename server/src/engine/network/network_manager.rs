use std::net::SocketAddr;

use tokio::sync::mpsc::{Receiver, Sender};
use webrtc_unreliable::Server as RtcServer;
use webrtc_unreliable::{MessageResult, MessageType};

use futures::{future::FutureExt, select, stream::StreamExt};

use super::client_data::ClientData;
use crate::engine::messaging::messages::*;

pub struct NetworkManager {
    clients: Vec<ClientData>, //Todo: Change to a hash map?
    ws_in: Receiver<WSClientMessage>,
    game_sender: Sender<GameMessage>,
    reliable_out_queue: Receiver<(OutTarget, OutMessage)>,
    unreliable_out_queue: Receiver<(OutTarget, OutMessage)>,
    rtc_server: RtcServer,
}

impl NetworkManager {
    pub fn new(
        ws_in: Receiver<WSClientMessage>,
        game_sender: Sender<GameMessage>,
        reliable_out_queue: Receiver<(OutTarget, OutMessage)>,
        unreliable_out_queue: Receiver<(OutTarget, OutMessage)>,
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
                ws_result = self.ws_in.recv().fuse() => match ws_result {
                    Some(ws_msg) => on_ws_in_msg(ws_msg, &mut self.clients, &mut self.game_sender),
                    None => (),
                },
                rtc_msg = self.rtc_server.recv(&mut msg_buf).fuse() => match rtc_msg {
                    Ok(msg) => on_rtc_in_msg(msg, &msg_buf, &mut self.clients, &mut self.game_sender),
                    Err(e) => println!("rtc server: {}", e),
                },
                reliable_result = self.reliable_out_queue.recv().fuse() => match reliable_result {
                    Some((targets, out_reliable)) =>
                        handle_reliable_out_msg(
                            get_targets(targets, &self.clients),
                            out_reliable,
                            &self.clients
                        ),
                    None => (),
                },
                unreliable_result = self.unreliable_out_queue.recv().fuse() => match unreliable_result {
                    Some((targets, out_unreliable)) =>
                        handle_unreliable_out_msg(
                            get_targets(targets, &self.clients),
                            out_unreliable,
                            &mut self.rtc_server,
                            &self.clients
                        ).await,
                    None => (),
                },

            }
        }
    }
}

fn on_ws_in_msg(
    in_msg: WSClientMessage,
    clients: &mut Vec<ClientData>,
    game_sender: &mut Sender<GameMessage>,
) {
    match in_msg {
        WSClientMessage::Connected(client) => {
            game_sender
                .try_send(GameMessage::ClientConnected(client.id))
                .unwrap();
            clients.push(client);
        }
        WSClientMessage::Disconnected(disc_id) => {
            clients.retain(|client| disc_id != client.id);
            game_sender
                .try_send(GameMessage::ClientDisconnected(disc_id))
                .unwrap();
        }
        WSClientMessage::Packet(id, data) => {
            //TODO Serealize Data and do stuff!!
        }
    };
}

fn on_rtc_in_msg(
    msg: MessageResult,
    msg_buf: &Vec<u8>,
    clients: &mut Vec<ClientData>,
    game_out: &mut Sender<GameMessage>,
) {
    //todo read the message, handle deserealizing
    println!("got rtc message from: {}", msg.remote_addr);
    let msg_text = &msg_buf[0..msg.message_len];
    let msg_string = std::str::from_utf8(&msg_text).unwrap_or("UNKNOWN");
    println!("{}", msg_string);

    //todo remove this later once i
    if let Some(client) = clients
        .iter_mut()
        .find(|client| client.socket_uuid == msg_string && client.socket_addr == None)
    {
        println!("User found!");
        client.socket_addr = Some(msg.remote_addr);
        client
            .ws_client_out
            .send(rmp_serde::to_vec(&OutMessage::VerifiedUuid).unwrap());
    }
}

fn handle_reliable_out_msg(
    out_indexes: Vec<usize>,
    out_msg: OutMessage,
    clients: &Vec<ClientData>,
) {
    //change to byte output
    for idx in out_indexes {
        let output = rmp_serde::to_vec(&out_msg).unwrap();
        //print!("{:?}", output);
        clients
            .get(idx)
            .unwrap()
            .ws_client_out
            .send(output)
            .unwrap();
    }
}

async fn handle_unreliable_out_msg(
    out_indexes: Vec<usize>,
    out_msg: OutMessage,
    rtc_server: &mut RtcServer,
    clients: &Vec<ClientData>,
) {
    for idx in out_indexes {
        let client = clients.get(idx).unwrap();
        if let Some(addr) = client.socket_addr {
            rtc_server
                .send(
                    &rmp_serde::to_vec(&out_msg).unwrap(),
                    MessageType::Binary,
                    &addr,
                )
                .await;
        }
    }
}

//Todo: Change to a hash map to optimize
fn get_targets(targets: OutTarget, clients: &Vec<ClientData>) -> Vec<usize> {
    match targets {
        OutTarget::All => {
            let mut out = Vec::new();
            for (i, client) in clients.iter().enumerate() {
                out.push(i);
            }
            out
        }
        OutTarget::Many(ids) => {
            let mut out = Vec::new();

            for (i, client) in clients
                .iter()
                .filter(|client| ids.iter().any(|id| id == &client.id))
                .enumerate()
            {
                out.push(i);
            }
            out
        }
        OutTarget::Single(id) => match clients.iter().position(|client| client.id == id) {
            Some(idx) => vec![idx],
            None => Vec::new(),
        },
    }
}