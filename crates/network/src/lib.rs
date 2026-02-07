use tokio::{net::TcpStream, sync::broadcast};

use crate::player_connection::PlayerConnection;

pub mod player_connection;

pub mod states;

pub async fn handle_connection(socket: TcpStream, rx: broadcast::Receiver<()>) {
    let mut connection = PlayerConnection::new(socket, rx);
    connection.run().await;
}
