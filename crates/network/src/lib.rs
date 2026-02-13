use std::sync::Arc;

use events::EventBus;
use tokio::{net::TcpStream, sync::broadcast};

use crate::player_connection::PlayerConnection;

pub mod player_connection;

pub mod event;
pub mod packets;
pub mod states;

pub async fn handle_connection(
    socket: TcpStream,
    rx: broadcast::Receiver<()>,
    event_bus: Arc<EventBus>,
) {
    let mut connection = PlayerConnection::new(socket, rx, event_bus);
    connection.run().await;
}
