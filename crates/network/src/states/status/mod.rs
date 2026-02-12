use std::io::Cursor;

use async_trait::async_trait;
use log::debug;
use protocol::serial::{PacketError, PacketRead};

use crate::packets::PacketHandler;

use crate::{
    PlayerConnection,
    packets::server::status::{StatusRequestPacket, ping::PingRequestPacket},
    states::PacketDispatcher,
};

#[derive(Debug)]
pub enum StatusPacketHandler {
    StatusRequest(StatusRequestPacket),
    Ping(PingRequestPacket),
}

impl StatusPacketHandler {
    pub fn from_id(id: i32, data: &mut Cursor<&[u8]>) -> Result<Self, PacketError> {
        debug!("Received status packet with id {}", id);
        match id {
            0x00 => Ok(StatusPacketHandler::StatusRequest(
                StatusRequestPacket::read(data)?,
            )),
            0x01 => Ok(StatusPacketHandler::Ping(PingRequestPacket::read(data)?)),
            _ => Err(PacketError::UnknownPacket),
        }
    }
}

#[async_trait]
impl PacketDispatcher for StatusPacketHandler {
    async fn dispatch_packet(
        &mut self,
        player_connection: &mut PlayerConnection,
    ) -> Result<(), PacketError> {
        match self {
            StatusPacketHandler::StatusRequest(packet) => {
                debug!("Received status request packet");
                packet.handle(player_connection).await?;
            }
            StatusPacketHandler::Ping(packet) => {
                debug!("Received ping request packet");
                packet.handle(player_connection).await?;
            }
        }
        Ok(())
    }
}
