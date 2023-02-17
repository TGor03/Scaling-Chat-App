#[non_exhaustive]
pub struct PacketType;

impl PacketType {
    pub const HELLO: u8 = 11;
    pub const HELLO_ACK: u8 = 12;
    pub const TEXT: u8 = 13;
}
