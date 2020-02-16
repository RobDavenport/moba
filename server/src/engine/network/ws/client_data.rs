extern crate ws;

#[derive(Debug)]
pub struct ClientData {
    pub id: u32,
    pub client_out: ws::Sender,
}
