use std::io::Cursor;

use crate::packets::{
    PacketHandler, bidirectional::configuration::FinishConfigurationPacket,
    server::configuration::ClientInformationPacket,
};

use async_trait::async_trait;
use protocol::serial::{PacketError, PacketRead};

use crate::{PlayerConnection, states::PacketDispatcher};

pub enum ConfigurationPacketHandler {
    ClientInformation(ClientInformationPacket),
    FinishConfiguration(FinishConfigurationPacket),
}

impl ConfigurationPacketHandler {
    pub fn from_id(id: i32, data: &mut Cursor<&[u8]>) -> Result<Self, PacketError> {
        match id {
            0x00 => {
                let packet = ClientInformationPacket::read(data)?;

                Ok(ConfigurationPacketHandler::ClientInformation(packet))
            }
            0x02 => {
                let packet = FinishConfigurationPacket::read(data)?;

                Ok(ConfigurationPacketHandler::FinishConfiguration(packet))
            }
            _ => Err(PacketError::UnknownPacket),
        }
    }
}

#[async_trait]
impl PacketDispatcher for ConfigurationPacketHandler {
    async fn dispatch_packet(
        &mut self,
        player_connection: &mut PlayerConnection,
    ) -> Result<(), PacketError> {
        match self {
            ConfigurationPacketHandler::ClientInformation(packet) => {
                packet.handle(player_connection).await?;
            }
            ConfigurationPacketHandler::FinishConfiguration(packet) => {
                packet.handle(player_connection).await?;
            }
        }
        Ok(())
    }
}
