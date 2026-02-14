use std::io::Cursor;

use async_trait::async_trait;
use components::{Component, get_protocol_version};
use protocol::serial::{PacketError, PacketRead};

use crate::{
    PlayerConnection,
    packets::{
        PacketHandler,
        client::login::LoginDisconnectionPacket,
        server::login::{LoginStartPacket, acknowledged::LoginAcknowledgedPacket},
    },
    states::PacketDispatcher,
};

pub enum LoginPacketHandler {
    LoginStart(LoginStartPacket),
    Acknowledged(LoginAcknowledgedPacket),
    LoginDisconnect(LoginDisconnectionPacket),
}

impl LoginPacketHandler {
    pub async fn from_id(
        ctx: &mut PlayerConnection,
        id: i32,
        data: &mut Cursor<&[u8]>,
    ) -> Result<Self, PacketError> {
        if ctx.get_protocol() != get_protocol_version() {
            let reason = Component::legacy_text("&cInvalid version please update your client.")
                .protocol(ctx.get_protocol());

            let kick = LoginDisconnectionPacket {
                reason: reason.to_json(),
            };
            return Ok(LoginPacketHandler::LoginDisconnect(kick));
        }

        match id {
            0x00 => Ok(LoginPacketHandler::LoginStart(LoginStartPacket::read(
                data,
            )?)),
            0x03 => Ok(LoginPacketHandler::Acknowledged(
                LoginAcknowledgedPacket::read(data)?,
            )),
            _ => Err(PacketError::UnknownPacket),
        }
    }
}

#[async_trait]
impl PacketDispatcher for LoginPacketHandler {
    async fn dispatch_packet(
        &mut self,
        player_connection: &mut PlayerConnection,
    ) -> Result<(), PacketError> {
        match self {
            LoginPacketHandler::LoginStart(packet) => packet.handle(player_connection).await?,
            LoginPacketHandler::Acknowledged(packet) => packet.handle(player_connection).await?,
            LoginPacketHandler::LoginDisconnect(packet) => {
                player_connection.send_packet(packet).await?
            }
        }
        Ok(())
    }
}
