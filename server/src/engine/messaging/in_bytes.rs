use std::net::SocketAddr;

pub struct InBytes {
 source: SourceIdentifier, 
 data: Vec<u8> 
}

pub enum SourceIdentifier {
  WebRTC(SocketAddr),
  WsID(u32)
}