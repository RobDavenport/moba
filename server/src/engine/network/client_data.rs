//use ws::Sender;
use futures::stream::SplitSink;

#[derive(Debug)]
pub struct ClientData {
    //pub id: u32,
    pub ws_client_out: SplitSink<warp::filters::ws::WebSocket, warp::filters::ws::Message>,
    pub socket_addr: Option<std::net::SocketAddr>,
    pub socket_uuid: String,
}
