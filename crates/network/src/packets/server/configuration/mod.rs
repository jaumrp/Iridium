use async_trait::async_trait;
use log::warn;
use macros::Packet;
use protocol::{serial::PacketError, types::var_int::VarInt};

use crate::{packets::PacketHandler, player_connection::PlayerConnection};

#[derive(Packet)]
pub struct ClientInformationPacket {
    pub locale: String,
    pub view_distance: i8,
    pub chat_mode: VarInt,
    pub chat_colors: bool,
    pub displayed_skin_parts: u8,
    pub main_hand: VarInt,
    pub enable_text_filtering: bool,
    pub allow_server_listing: bool,
}

#[async_trait]
impl PacketHandler for ClientInformationPacket {
    async fn handle(&mut self, _ctx: &mut PlayerConnection) -> Result<(), PacketError> {
        warn!("{}", &self.locale);
        Ok(())
    }
}
