use async_trait::async_trait;
use components::{Component, colors::Color};
use macros::Packet;

use crate::{
    packets::{PlayerContext, client::status::StatusBuilder},
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
        let packet = StatusBuilder::new().protocol(ctx.get_protocol()).build();

        ctx.send_packet(&packet).await?;

        Ok(())
    }
}
