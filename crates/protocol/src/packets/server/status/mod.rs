use async_trait::async_trait;
use components::Component;
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
        let packet = StatusBuilder::new()
            .motd(Component::modern_text_as_protocol(
                "<red>reds<green>greens<bold><blue>blue\n<gradient:red:blue>Hello World</gradient>",
                ctx.get_protocol(),
            ))
            .build();

        ctx.send_packet(&packet).await?;

        Ok(())
    }
}
