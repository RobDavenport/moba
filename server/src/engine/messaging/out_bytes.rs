use std::net::SocketAddr;

pub struct OutBytes(Vec<u8>);

pub struct UnreliableOutBytes((SocketAddr, OutBytes));
