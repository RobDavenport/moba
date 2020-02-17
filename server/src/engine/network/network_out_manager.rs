use std::collections::VecDeque;
use std::net::SocketAddr;

use tokio::sync::mpsc::Sender;

use futures::join;

use super::client_data::ClientData;
use crate::engine::messaging::messages::OutMessage;

pub struct NetworkOutManager {
    clients: Vec<ClientData>,
    reliable_out_queue: VecDeque<(u32, OutMessage)>,
    unreliable_out_queue: VecDeque<(u32, OutMessage)>,
    game_out_reliable: Sender<OutMessage>,
    game_out_unreliable: Sender<OutMessage>,
}

impl NetworkOutManager {
    pub fn new(
        game_out_reliable: Sender<OutMessage>,
        game_out_unreliable: Sender<OutMessage>,
    ) -> Self {
        Self {
            clients: Vec::new(),
            reliable_out_queue: VecDeque::new(),
            unreliable_out_queue: VecDeque::new(),
            game_out_reliable,
            game_out_unreliable,
        }
    }

    pub async fn process(&mut self) {
        join!(
            process_reliable(&mut self.reliable_out_queue, &mut self.game_out_reliable),
            process_unreliable(
                &mut self.unreliable_out_queue,
                &mut self.game_out_unreliable
            )
        );
    }

    pub fn send_reliable(&mut self, target: u32, out_msg: OutMessage) {
        self.reliable_out_queue.push_back((target, out_msg));
    }

    pub fn send_unreliable(&mut self, target: u32, out_msg: OutMessage) {
        self.unreliable_out_queue.push_back((target, out_msg));
    }

    // match self.clients.iter().find(|client| client.id == target) {
    //   Some(out_client) => {
    //     self.unreliable_out.push_back((out_client.socket_addr, out_msg))
    //   },
    //   None => println!("Unreliable client not found!")
    // };
}

async fn process_reliable(
    out_queue: &mut VecDeque<(u32, OutMessage)>,
    game_out: &mut Sender<OutMessage>,
) {
    while let Some(reliable_msg) = out_queue.pop_front() {
        game_out.send(reliable_msg.1).await; //process these correctly
    }
}

async fn process_unreliable(
    out_queue: &mut VecDeque<(u32, OutMessage)>,
    game_out: &mut Sender<OutMessage>,
) {
    while let Some(unreliable_msg) = out_queue.pop_front() {
        game_out.send(unreliable_msg.1).await; // process these correctly
    }
}
