use ws::Sender;

#[derive(Debug)]
pub struct ClientData {
    pub id: u32,
    pub ws_client_out: Sender,
    pub socket_addr: Option<std::net::SocketAddr>,
    pub socket_uuid: String,
}
