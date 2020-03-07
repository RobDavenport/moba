// use tokio::sync::mpsc::Sender;

// use uuid::Uuid;
// use ws;

// use super::client::Client;

// use crate::engine::messaging::messages::{OutMessage, WSClientMessage};
// use crate::engine::network::client_data::ClientData;

// use crate::engine::components::player_controlled::PlayerId;
// use crate::engine::network::out_message_builder::build_out_message;

// pub struct ClientFactory {
//     client_sender: Sender<WSClientMessage>,
//     next_client_id: u32,
// }

// impl ClientFactory {
//     pub fn new(client_sender: Sender<WSClientMessage>) -> Self {
//         Self {
//             client_sender,
//             next_client_id: 0,
//         }
//     }
// }

// impl ws::Factory for ClientFactory {
//     type Handler = Client;

//     fn connection_made(&mut self, out: ws::Sender) -> Client {
//         let new_client_id = PlayerId(self.next_client_id);
//         self.next_client_id += 1;
//         let new_client = Client {
//             manager_out: self.client_sender.clone(),
//             id: new_client_id,
//         };

//         let uuid = Uuid::new_v4().to_string();

//         self.client_sender.try_send(WSClientMessage::Connected(
//             new_client_id,
//             ClientData {
//                 ws_client_out: out.clone(),
//                 socket_addr: None,
//                 socket_uuid: uuid.clone(),
//             },
//         ));

//         println!("sending uuid: {}", &uuid);
//         out.send(build_out_message(OutMessage::VerifyUuid(uuid)));

//         new_client
//     }
// }
