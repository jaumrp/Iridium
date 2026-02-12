use async_trait::async_trait;
use protocol::serial::PacketError;

use crate::player_connection::PlayerConnection;

pub mod client;
pub mod server;

#[async_trait]
pub trait PacketHandler {
    async fn handle(&mut self, ctx: &mut PlayerConnection) -> Result<(), PacketError>;
}
