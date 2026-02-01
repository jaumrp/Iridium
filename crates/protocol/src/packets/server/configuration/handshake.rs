use macros::Packet;

use crate::types::var_int::VarInt;

#[derive(Packet, Debug)]
pub struct HandshakePacket {
    pub protocol_version: VarInt,
    pub server_address: String,
    pub server_port: u16,
    pub next_state: VarInt,
}
