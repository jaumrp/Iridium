use crate::{
    event::server::ServerListPingEvent,
    packets::{PacketHandler, client::status::StatusBuilder},
    player_connection::PlayerConnection,
};
use async_trait::async_trait;
use components::Component;
use macros::Packet;
use protocol::serial::PacketError;

pub mod ping;

#[derive(Packet, Debug)]
pub struct StatusRequestPacket {}

#[async_trait]
impl PacketHandler for StatusRequestPacket {
    async fn handle(&mut self, ctx: &mut PlayerConnection) -> Result<(), PacketError> {
        let mut status = StatusBuilder::new();
        status.protocol(ctx.get_protocol());
        status.motd(Component::modern_text_as_protocol(
            "<red>reds<green>greens<bold><blue>blue\n<gradient:red:blue>Hello World</gradient>",
            ctx.get_protocol(),
        ));

        let mut my_event = ServerListPingEvent::new(status);
        match ctx.event_bus().emit::<ServerListPingEvent>(&mut my_event) {
            Ok(_) => {
                let packet = my_event.status.build();
                ctx.send_packet(&packet).await?;
            }
            Err(err) => {
                log::error!("Failed to emit ServerListPingEvent: {}", err);
            }
        };

        Ok(())
    }
}
