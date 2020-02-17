use std::net::SocketAddr;

use tokio::sync::mpsc::{ Sender, Receiver };

use futures::{future::FutureExt, select, stream::StreamExt};

use super::client_data::ClientData;
use crate::engine::messaging::messages::{ 
    OutMessage, OutBytes, OutBytesUnreliable, InBytes, SourceIdentifier 
};

//todo: hook up game.rs to use this instead, and engine builders
pub struct NetworkManager {
    clients: Vec<ClientData>,
    client_in: Receiver<InBytes>,
    reliable_out_queue: Receiver<(u32, OutMessage)>,
    unreliable_out_queue: Receiver<(u32, OutMessage)>,
    reliable_out_bytes: Sender<OutBytes>,
    unreliable_out_bytes: Sender<OutBytesUnreliable>,
}

impl NetworkManager {
    pub fn new(
        client_in: Receiver<InBytes>,
        game_out_reliable: Sender<OutBytes>,
        game_out_unreliable: Sender<OutBytesUnreliable>,
        reliable_out_queue: Receiver<(u32, OutMessage)>,
        unreliable_out_queue: Receiver<(u32, OutMessage)>,
        reliable_out_bytes: Sender<OutBytes>,
        unreliable_out_bytes: Sender<OutBytesUnreliable>
    ) -> Self {
        Self {
            clients: Vec::new(),
            client_in,
            reliable_out_queue,
            unreliable_out_queue,
            reliable_out_bytes,
            unreliable_out_bytes,
        }
    }

    pub async fn process(&mut self) {
        loop {
            select! {
                in_result = self.client_in.recv().fuse() => match in_result {
                    Some(in_msg) => handle_in_msg(in_msg, &mut self.clients),
                    None => (),
                },
                reliable_result = self.reliable_out_queue.recv().fuse() => match reliable_result {
                    Some((target, out_reliable)) => handle_reliable_msg(target, out_reliable, &self.clients, &mut self.reliable_out_bytes),
                    None => (),
                },
                unreliable_result = self.unreliable_out_queue.recv().fuse() => match unreliable_result {
                    Some((target, out_unreliable)) => handle_unreliable_msg(target, out_unreliable, &self.clients, &mut self.unreliable_out_bytes),
                    None => (),
                },
            }
        }
    }
}

fn handle_in_msg(in_msg: InBytes, clients: &mut Vec<ClientData>) {
    //unpack the message
    //process it
}

fn handle_reliable_msg(
    id: u32, 
    out_msg: OutMessage, 
    clients: &Vec<ClientData>, 
    reliable_sender: &mut Sender<OutBytes>) {
    match clients.iter().find(|client| client.id == id) {
        Some(client) => (), //TODO
        None => (),
    }
}

fn handle_unreliable_msg(
    id: u32, 
    out_msg: OutMessage, 
    clients: &Vec<ClientData>, 
    unreliable_sender: &mut Sender<OutBytesUnreliable>) {
    match clients.iter().find(|client| client.id == id) {
        Some(client) => (), //TODO
        None => (),
    }
}

