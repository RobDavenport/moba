// use std::thread;

// use tokio::sync::mpsc::Sender;

// use super::client_factory::ClientFactory;
// use crate::engine::messaging::messages::WSClientMessage;
// use ws::WebSocket;

// pub fn start_ws_server(
//     address: String,
//     ws_client_sender: Sender<WSClientMessage>,
// ) -> thread::JoinHandle<()> {
//     let ws_thread = thread::spawn(|| {
//         let client_factory = ClientFactory::new(ws_client_sender);
//         let ws_server = WebSocket::<ClientFactory>::new(client_factory).unwrap();
//         println!("New WS server active at: {}", address);
//         ws_server.listen(address).unwrap();
//     });

//     ws_thread
// }
