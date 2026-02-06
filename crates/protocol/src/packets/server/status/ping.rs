use async_trait::async_trait;
use log::debug;
use macros::Packet;

use crate::{
    packets::PlayerContext,
    serial::{PacketError, PacketHandler},
};

#[derive(Packet, Debug)]
#[packet(id = 0x01)]
pub struct PingResponsePacket {
    pub payload: i64,
}

#[derive(Packet, Debug)]
#[packet(id = 0x01)]
pub struct PingRequestPacket {
    pub payload: i64,
}

#[async_trait]
impl PacketHandler for PingRequestPacket {
    async fn handle<Context: PlayerContext>(
        &mut self,
        ctx: &mut Context,
    ) -> Result<(), PacketError> {
        debug!("Received ping response packet");
        let response = PingResponsePacket {
            payload: self.payload,
        };

        ctx.send_packet(&response).await?;

        Ok(())
    }
}
