use std::net::TcpStream;
use uuid::Uuid;

#[non_exhaustive]
#[derive(Debug)]
pub struct PacketType;

impl PacketType {
    pub const HELLO: u8 = 11;
    pub const HELLO_ACK: u8 = 12;
    pub const TEXT: u8 = 42;
}

//Struct representing client data
#[derive(Debug)]
pub struct Client {
    pub id: Uuid,
    pub stream: TcpStream,
}
