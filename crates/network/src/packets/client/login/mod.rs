use macros::Packet;
use protocol::types::property::Property;

#[derive(Packet)]
#[packet(id = 0x02)]
pub struct LoginSuccessPacket {
    pub uuid: uuid::Uuid,
    pub name: String,
    pub properties: Vec<Property>,
}

#[derive(Packet)]
pub struct LoginDisconnectionPacket {
    pub reason: String,
}
