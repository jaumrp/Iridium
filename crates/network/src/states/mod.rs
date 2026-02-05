use std::io::Cursor;

use async_trait::async_trait;
use protocol::serial::PacketError;

use crate::PlayerConnection;

pub mod handshaking;
pub mod login;
pub mod play;
pub mod status;

#[async_trait]
pub trait ProtocolState {
    async fn handle_packet(
        &mut self,
        player_connection: &mut PlayerConnection,
        packet_id: i32,
        data: Cursor<&[u8]>,
    ) -> Result<(), PacketError>;
}
