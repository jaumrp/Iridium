use std::io::Cursor;

use bytes::{Buf, BytesMut};
use log::{error, info, warn};
use protocol::{
    packets::server::configuration::handshake::HandshakePacket,
    serial::{PacketError, PacketRead},
    types::var_int::VarInt,
};
use tokio::{io::AsyncReadExt, net::TcpStream, sync::broadcast};

pub mod states;

pub enum ConnectionState {
    Handshaking,
    Login,
    Status,
    Play,
}

pub struct PlayerConnection {
    socket: TcpStream,
    buffer: BytesMut,
    state: ConnectionState,
    shutdown_tx: broadcast::Receiver<()>,
}

impl PlayerConnection {
    pub fn new(socket: TcpStream, shutdown_tx: broadcast::Receiver<()>) -> Self {
        PlayerConnection {
            socket,
            buffer: BytesMut::with_capacity(4096),
            state: ConnectionState::Handshaking,
            shutdown_tx,
        }
    }

    pub async fn run(&mut self) {
        loop {
            tokio::select! {
                    _ = self.shutdown_tx.recv() => {
                        // encerrar conexão
                        break;
                    }
                    read = self.socket.read_buf(&mut self.buffer) => {
                        match read {
                            Err(e) => {
                                error!("Error reading from socket: {}", e);
                                break;
                            },
                            Ok(n) if n == 0 => return,
                            Ok(_) => {
                                loop {
                                    let mut cursor = Cursor::new(&self.buffer[..]);

                                    let packet_len = match VarInt::read(&mut cursor) {
                                        Ok(i) => i.0 as usize,
                                        Err(_) => {
                                            break;
                                        }
                                    };
                                    let len = cursor.position() as usize;
                                    if self.buffer.len() < len + packet_len {
                                        if self.buffer.capacity() < len + packet_len {
                                            self.buffer.reserve(len + packet_len);
                                        }
                                        break;
                                    }

                                    self.buffer.advance(len);
                                    let mut packet_data = self.buffer.split_to(packet_len);

                                    if let Err(e) = self.handle_packet(&mut packet_data).await {
                                        match e {
                                            PacketError::Incomplete => {}
                                            _ => error!("Error handling packet: {}", e),
                                        }
                                        return;
                                    }
                                }
                            }
                        }
                    }
            }
        }
    }
    async fn handle_packet(&mut self, packet_data: &mut BytesMut) -> Result<(), PacketError> {
        let mut cursor = Cursor::new(&packet_data[..]);
        let _packet_id = VarInt::read(&mut cursor)?.0;

        //info!("received packet with id {}", packet_id);

        match self.state {
            ConnectionState::Handshaking => {
                let packet = HandshakePacket::read(&mut cursor)?;
                info!("{:?}", packet);
            }
            ConnectionState::Status => {
                warn!("state is not implemented");
            }
            ConnectionState::Login => {
                warn!("state is not implemented");
            }
            ConnectionState::Play => {
                warn!("state is not implemented");
            }
            _ => {
                warn!("state is not implemented");
            }
        }

        Ok(())
    }
}

pub async fn handle_connection(socket: TcpStream, rx: broadcast::Receiver<()>) {
    let mut connection = PlayerConnection::new(socket, rx);
    connection.run().await;
}
