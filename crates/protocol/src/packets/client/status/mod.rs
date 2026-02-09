use components::{Component, colors::Color, get_protocol_version, get_version_name};
use macros::Packet;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::types::var_int::VarInt;

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
    enfocers_secure_chat: Option<bool>,
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
            enfocers_secure_chat: Some(false),
        }
    }

    pub fn version<Version: Into<String>>(mut self, name: Version, protocol: i32) -> Self {
        self.version_name = name.into();
        self.protocol = VarInt(protocol);
        self
    }

    pub fn max_players(mut self, max_players: i32) -> Self {
        self.max_players = VarInt(max_players);
        self
    }

    pub fn online_players(mut self, online_players: i32) -> Self {
        self.online_players = VarInt(online_players);
        self
    }

    pub fn players(mut self, online_players: i32, max_players: i32) -> Self {
        self.online_players = VarInt(online_players);
        self.max_players = VarInt(max_players);
        self
    }

    pub fn add_sample<Sample: Into<String>>(mut self, sample: Sample) -> Self {
        self.sample.push(PlayerSample {
            name: sample.into(),
            id: "00000000-0000-0000-0000-000000000000".into(), // uuid
        });
        self
    }

    pub fn sample(mut self, sample: Vec<PlayerSample>) -> Self {
        self.sample = sample;
        self
    }

    pub fn motd(mut self, motd: Component) -> Self {
        self.motd = motd;
        self
    }

    pub fn protocol(mut self, protocol: i32) -> Self {
        self.motd.protocol = protocol;
        self
    }

    pub fn favicon(mut self, favicon: Option<String>) -> Self {
        self.favicon = favicon;
        self
    }

    pub fn enfocers_secure_chat(mut self, enfocers_secure_chat: Option<bool>) -> Self {
        self.enfocers_secure_chat = enfocers_secure_chat;
        self
    }

    pub fn build(self) -> StatusResponsePacket {
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
            "enforcersSecureChat": self.enfocers_secure_chat,
            "description": self.motd,
        });

        if let Some(favicon) = self.favicon {
            json["favicon"] = favicon.into();
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
