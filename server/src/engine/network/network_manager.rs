use std::collections::HashMap;

use tokio::sync::mpsc::{Receiver, Sender};
use webrtc_unreliable::Server as RtcServer;
use webrtc_unreliable::{MessageResult, MessageType};

use futures::{future::FutureExt, select};
use futures_util::sink::SinkExt;

use super::client_data::ClientData;
use crate::engine::messaging::messages::*;

use crate::engine::components::player_controlled::PlayerId;

use super::in_message_reader::handle_client_command;
use super::protobuf::ClientMessage::*; //todo cut this in favor of reader?

//TODO: add hash for "game/match ID?"
use warp::filters::ws::Message;
pub struct NetworkManager {
    clients: HashMap<PlayerId, ClientData>, //Todo: Change to a hash map of hashmaps?
    unreliable_ids: HashMap<std::net::SocketAddr, PlayerId>, //Combine these into a "Socket Manager?"
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
            clients: HashMap::new(),
            unreliable_ids: HashMap::new(),
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
                    Ok(msg) => on_rtc_in_msg(msg, &msg_buf, &mut self.clients, &mut self.game_sender, &mut self.unreliable_ids).await,
                    Err(e) => println!("rtc server: {}", e),
                },
                reliable_result = self.reliable_out_queue.recv().fuse() => match reliable_result {
                    Some((targets, out_reliable)) =>
                        handle_reliable_out_msg(
                            get_targets(targets, &self.clients),
                            out_reliable,
                            &mut self.clients
                        ).await,
                    None => (),
                },
                unreliable_result = self.unreliable_out_queue.recv().fuse() => match unreliable_result {
                    Some((targets, out_unreliable)) =>
                        handle_unreliable_out_msg(
                            get_targets(targets, &self.clients),
                            out_unreliable,
                            &mut self.rtc_server,
                            &self.clients,
                        ).await,
                    None => (),
                },
            }
        }
    }
}

fn on_ws_in_msg(
    in_msg: WSClientMessage,
    clients: &mut HashMap<PlayerId, ClientData>,
    game_sender: &mut Sender<GameMessage>,
) {
    match in_msg {
        WSClientMessage::Connected(client_id, client_data) => {
            game_sender
                .try_send(GameMessage::ClientConnected(client_id))
                .unwrap();
            clients.insert(client_id, client_data);
        }
        WSClientMessage::Disconnected(disc_id) => {
            clients.remove(&disc_id);
            game_sender
                .try_send(GameMessage::ClientDisconnected(disc_id))
                .unwrap();
        }
        WSClientMessage::Packet(id, data) => {
            if let Ok(protomsg) = protobuf::parse_from_bytes::<ClientMessage>(&data) {
                if let Some(protomsgtype) = protomsg.msgData {
                    match protomsgtype {
                        ClientMessage_oneof_msgData::command(command_msg) => {
                            if let Some(out_msg) = handle_client_command(command_msg, id) {
                                game_sender.try_send(out_msg);
                            };
                        }
                        ClientMessage_oneof_msgData::veryfiyRtc(..) => (),
                        ClientMessage_oneof_msgData::ack(..) => (),
                    }
                }
            }
        }
    };
}

async fn on_rtc_in_msg(
    msg: MessageResult,
    msg_buf: &Vec<u8>,
    clients: &mut HashMap<PlayerId, ClientData>,
    game_out: &mut Sender<GameMessage>,
    unreliable_ids: &mut HashMap<std::net::SocketAddr, PlayerId>,
) {
    //todo read the message, handle deserealizing
    let msg_text = &msg_buf[0..msg.message_len];
    if let Ok(protomsg) = protobuf::parse_from_bytes::<ClientMessage>(msg_text) {
        if let Some(protomsgtype) = protomsg.msgData {
            match protomsgtype {
                ClientMessage_oneof_msgData::veryfiyRtc(verify_rtc_msg) => {
                    println!("got uuid: {}", &verify_rtc_msg.uuid);
                    for (key, val) in clients.iter_mut() {
                        if val.socket_uuid == verify_rtc_msg.uuid && val.socket_addr == None {
                            println!("User found!");
                            val.socket_addr = Some(msg.remote_addr);
                            unreliable_ids.insert(msg.remote_addr, *key);
                            val.ws_client_out
                                .send(Message::binary(OutMessage::VerifiedUuid.to_proto_bytes()))
                                .await;
                            break;
                        }
                    }
                }
                ClientMessage_oneof_msgData::command(command_msg) => {
                    if let Some(player_id) = unreliable_ids.get(&msg.remote_addr) {
                        if let Some(out_msg) = handle_client_command(command_msg, *player_id) {
                            game_out.try_send(out_msg);
                        };
                    }
                }
                ClientMessage_oneof_msgData::ack(ack_msg) => {
                    if let Some(player_id) = unreliable_ids.get(&msg.remote_addr) {
                        game_out.try_send(GameMessage::Ack {
                            id: *player_id,
                            new_baseline: ack_msg.newBaseline,
                        });
                    }
                }
            }
        }
    }
}

async fn handle_reliable_out_msg(
    out_indexes: Vec<PlayerId>,
    out_msg: OutMessage,
    clients: &mut HashMap<PlayerId, ClientData>,
) {
    let output = Message::binary(out_msg.to_proto_bytes());

    for idx in out_indexes {
        clients
            .get_mut(&idx)
            .unwrap()
            .ws_client_out
            .send(output.clone())
            .await;
    }
}

async fn handle_unreliable_out_msg(
    out_indexes: Vec<PlayerId>,
    out_msg: OutMessage,
    rtc_server: &mut RtcServer,
    clients: &HashMap<PlayerId, ClientData>,
) {
    let output = out_msg.to_proto_bytes();

    for idx in out_indexes {
        let client = clients.get(&idx).unwrap();
        if let Some(addr) = client.socket_addr {
            rtc_server.send(&output, MessageType::Binary, &addr).await;
        }
    }
}

fn get_targets(targets: OutTarget, clients: &HashMap<PlayerId, ClientData>) -> Vec<PlayerId> {
    match targets {
        OutTarget::All => clients.keys().cloned().collect(),
        OutTarget::Many(ids) => ids,
        OutTarget::Single(id) => vec![id],
    }
}
