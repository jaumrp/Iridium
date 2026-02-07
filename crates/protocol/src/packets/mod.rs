use async_trait::async_trait;

use crate::serial::{PacketError, PacketWrite};

pub mod client;
pub mod server;

pub enum ConnectionState {
    Handshaking,
    Login,
    Status,
    Play,
}

#[async_trait]
pub trait PlayerContext: Send + Sync {
    fn get_state(&self) -> &ConnectionState;
    fn set_state(&mut self, state: ConnectionState);
    fn get_protocol(&self) -> i32;
    fn set_protocol(&mut self, protocol: i32);
    async fn send_packet(&mut self, packet: &dyn PacketWrite) -> Result<(), PacketError>;
}
