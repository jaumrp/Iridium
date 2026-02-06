use macros::Packet;

#[derive(Packet, Debug)]
pub struct StatusResponsePacket {
    pub status: String,
}

impl StatusResponsePacket {
    pub fn new() -> Self {
        Self {
            status: r#"{
                "version": { "name": "1.21.1", "protocol": 773 },
                "players": { "max": 20, "online": 0 },
                "description": { "text": "§cOlá Mundo!" }
            }"#
            .to_string(),
        }
    }
}
