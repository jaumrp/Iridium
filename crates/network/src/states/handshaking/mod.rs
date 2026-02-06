use std::io::Cursor;

use async_trait::async_trait;
use protocol::{
    packets::server::configuration::handshake::HandshakePacket,
    serial::{PacketError, PacketHandler, PacketRead},
};

use crate::{PlayerConnection, states::PacketDispatcher};

#[derive(Debug)]
pub enum HandshakePacketHandler {
    Handshake(HandshakePacket),
}

impl HandshakePacketHandler {
    pub fn from_id(id: i32, data: &mut Cursor<&[u8]>) -> Result<Self, PacketError> {
        match id {
            0x00 => {
                let packet = HandshakePacket::read(data)?;

                Ok(HandshakePacketHandler::Handshake(packet))
            }
            _ => Err(PacketError::UnknownPacket),
        }
    }
}

#[async_trait]
impl PacketDispatcher for HandshakePacketHandler {
    async fn dispatch_packet(
        &mut self,
        player_connection: &mut PlayerConnection,
    ) -> Result<(), PacketError> {
        match self {
            HandshakePacketHandler::Handshake(packet) => {
                packet.handle(player_connection).await?;
            }
        }
        Ok(())
    }
}
