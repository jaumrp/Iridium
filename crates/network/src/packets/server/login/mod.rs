use async_trait::async_trait;
use log::debug;
use macros::Packet;
use protocol::serial::PacketError;

use crate::{
    packets::{PacketHandler, client::login::LoginSuccessPacket},
    player_connection::PlayerConnection,
};
pub mod acknowledged;

#[derive(Packet)]
#[packet(id = 0x00)]
pub struct LoginStartPacket {
    pub name: String,
    pub uuid: uuid::Uuid,
}

#[async_trait]
impl PacketHandler for LoginStartPacket {
    async fn handle(&mut self, ctx: &mut PlayerConnection) -> Result<(), PacketError> {
        debug!("Received login start packet");

        ctx.register(self.name.clone(), self.uuid);

        let sucess_packet = LoginSuccessPacket {
            name: self.name.clone(),
            uuid: self.uuid,
            properties: vec![],
        };

        ctx.send_packet(&sucess_packet).await?;

        Ok(())
    }
}
