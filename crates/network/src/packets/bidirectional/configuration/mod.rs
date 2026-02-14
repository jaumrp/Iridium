use async_trait::async_trait;
use macros::Packet;
use protocol::serial::PacketError;

use crate::{packets::PacketHandler, player_connection::PlayerConnection};

#[derive(Packet)]
#[packet(id = 0x02)]
pub struct FinishConfigurationPacket {}

#[async_trait]
impl PacketHandler for FinishConfigurationPacket {
    async fn handle(&mut self, ctx: &mut PlayerConnection) -> Result<(), PacketError> {
        ctx.set_state(protocol::ConnectionState::Play);

        Ok(())
    }
}
