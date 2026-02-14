use async_trait::async_trait;
use macros::Packet;
use protocol::serial::PacketError;

use crate::{packets::PacketHandler, player_connection::PlayerConnection};

#[derive(Packet)]
#[packet(id = 0x03)]
pub struct LoginAcknowledgedPacket {}

#[async_trait]
impl PacketHandler for LoginAcknowledgedPacket {
    async fn handle(&mut self, ctx: &mut PlayerConnection) -> Result<(), PacketError> {
        ctx.set_state(protocol::ConnectionState::Configuration);
        Ok(())
    }
}
