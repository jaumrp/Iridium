use async_trait::async_trait;
use log::debug;
use macros::Packet;

use crate::{
    packets::{PlayerContext, client::status::StatusResponsePacket},
    serial::{PacketError, PacketHandler},
};

pub mod ping;

#[derive(Packet, Debug)]
pub struct StatusRequestPacket {}

#[async_trait]
impl PacketHandler for StatusRequestPacket {
    async fn handle<Context: PlayerContext>(
        &mut self,
        ctx: &mut Context,
    ) -> Result<(), PacketError> {
        debug!("Handling StatusRequestPacket: {:?}", self);

        let packet = StatusResponsePacket::new();

        ctx.send_packet(&packet).await?;

        Ok(())
    }
}
