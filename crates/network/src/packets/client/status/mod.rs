use components::{Component, colors::Color, get_protocol_version, get_version_name};
use macros::Packet;
use protocol::types::var_int::VarInt;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize)]
pub struct PlayerSample {
    pub name: String,
    pub id: String,
}

pub struct StatusBuilder {
    version_name: String,
    protocol: VarInt,
    max_players: VarInt,
    online_players: VarInt,
    sample: Vec<PlayerSample>,
    motd: Component,
    favicon: Option<String>,
    enforcers_secure_chat: Option<bool>,
}

impl StatusBuilder {
    pub fn new() -> Self {
        Self {
            version_name: get_version_name(),
            protocol: VarInt(get_protocol_version()),
            max_players: VarInt(20),
            online_players: VarInt(0),
            sample: Vec::new(),
            motd: Component::modern_text("Iridium Server").color(Color::from("#692aa8").unwrap()),
            favicon: None,
            enforcers_secure_chat: Some(false),
        }
    }

    pub fn get_protocol_version(&self) -> VarInt {
        VarInt(self.motd.protocol)
    }

    pub fn version<Version: Into<String>>(mut self, name: Version, protocol: i32) -> Self {
        self.version_name = name.into();
        self.protocol = VarInt(protocol);
        self
    }

    pub fn max_players(&mut self, max_players: i32) -> &mut Self {
        self.max_players = VarInt(max_players);
        self
    }

    pub fn online_players(&mut self, online_players: i32) -> &mut Self {
        self.online_players = VarInt(online_players);
        self
    }

    pub fn players(&mut self, online_players: i32, max_players: i32) -> &mut Self {
        self.online_players = VarInt(online_players);
        self.max_players = VarInt(max_players);
        self
    }

    pub fn add_sample<Sample: Into<String>>(&mut self, sample: Sample) -> &mut Self {
        self.sample.push(PlayerSample {
            name: sample.into(),
            id: "00000000-0000-0000-0000-000000000000".into(), // uuid
        });
        self
    }

    pub fn sample(&mut self, sample: Vec<PlayerSample>) -> &mut Self {
        self.sample = sample;
        self
    }

    pub fn motd(&mut self, motd: Component) -> &mut Self {
        self.motd = motd;
        self
    }

    pub fn protocol(&mut self, protocol: i32) -> &mut Self {
        self.motd.protocol = protocol;
        self
    }

    pub fn favicon(&mut self, favicon: Option<String>) -> &mut Self {
        self.favicon = favicon;
        self
    }

    pub fn enforcers_secure_chat(&mut self, enforcers_secure_chat: Option<bool>) -> &mut Self {
        self.enforcers_secure_chat = enforcers_secure_chat;
        self
    }

    pub fn build(&mut self) -> StatusResponsePacket {
        let mut json = json!({
            "version": {
                "name": self.version_name,
                "protocol": self.protocol
            },
            "players": {
                "max": self.max_players,
                "online": self.online_players,
                "sample": self.sample
            },
            "enforcersSecureChat": self.enforcers_secure_chat,
            "description": self.motd,
        });

        if let Some(favicon) = &self.favicon {
            json["favicon"] = favicon.as_str().into();
        }

        StatusResponsePacket {
            pay_load: json.to_string(),
        }
    }
}

#[derive(Packet, Debug)]
pub struct StatusResponsePacket {
    pub pay_load: String,
}

//impl StatusResponsePacket {
//    pub fn new() -> Self {
//        Self {
//          pay_load: r#"{
//            "version": { "name": "1.21.11", "protocol": 774 },
//          "players": { "max": 20, "online": 0 },
//        "description": { "text": "§cOlá Mundo!" }
//  }"#
// .to_string(),
//}
//}
//}
