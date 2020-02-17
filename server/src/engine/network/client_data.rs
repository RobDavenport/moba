extern crate ws;

#[derive(Debug)]
pub struct ClientData {
    pub id: u32,
    pub ws_client_out: ws::Sender,
    pub socket_addr: Option<std::net::SocketAddr>,
}
