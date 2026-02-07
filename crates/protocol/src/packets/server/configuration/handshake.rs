use async_trait::async_trait;
use log::debug;
use macros::Packet;

use crate::{
    packets::PlayerContext,
    serial::{PacketError, PacketHandler},
    types::var_int::VarInt,
};

#[derive(Packet, Debug)]
pub struct HandshakePacket {
    pub protocol_version: VarInt,
    pub server_address: String,
    pub server_port: u16,
    pub next_state: VarInt,
}

#[async_trait]
impl PacketHandler for HandshakePacket {
    async fn handle<Context: PlayerContext>(
        &mut self,
        ctx: &mut Context,
    ) -> Result<(), PacketError> {
        ctx.set_protocol(self.protocol_version.0);

        match self.next_state.0 {
            1 => {
                ctx.set_state(crate::packets::ConnectionState::Status);
            }
            2 => {
                ctx.set_state(crate::packets::ConnectionState::Login);
            }
            3 => {
                debug!("Transfer is not supported");
            }
            _ => return Err(PacketError::InvalidData),
        }

        Ok(())
    }
}
