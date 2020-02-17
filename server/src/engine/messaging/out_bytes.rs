use std::net::SocketAddr;

pub struct OutBytes(Vec<u8>);

pub struct OutBytesUnreliable(SocketAddr, OutBytes);
