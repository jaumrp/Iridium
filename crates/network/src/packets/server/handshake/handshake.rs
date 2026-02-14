use async_trait::async_trait;
use log::debug;
use macros::Packet;
use protocol::{ConnectionState, serial::PacketError, types::var_int::VarInt};

use crate::{packets::PacketHandler, player_connection::PlayerConnection};

#[derive(Packet, Debug)]
pub struct HandshakePacket {
    pub protocol_version: VarInt,
    pub server_address: String,
    pub server_port: u16,
    pub next_state: VarInt,
}

#[async_trait]
impl PacketHandler for HandshakePacket {
    async fn handle(&mut self, ctx: &mut PlayerConnection) -> Result<(), PacketError> {
        ctx.set_protocol(self.protocol_version.0);

        match self.next_state.0 {
            1 => {
                ctx.set_state(ConnectionState::Status);
            }
            2 => {
                ctx.set_state(ConnectionState::Login);
            }
            3 => {
                debug!("Transfer is not supported");
            }
            _ => return Err(PacketError::InvalidData),
        }

        Ok(())
    }
}
