use async_trait::async_trait;
use protocol::serial::PacketError;

use crate::PlayerConnection;

pub mod handshaking;
pub mod login;
pub mod play;
pub mod status;

#[async_trait]
pub trait PacketDispatcher {
    async fn dispatch_packet(
        &mut self,
        player_connection: &mut PlayerConnection,
    ) -> Result<(), PacketError>;
}
