use std::io::Cursor;

use async_trait::async_trait;
use bytes::{Buf, BytesMut};
use log::{error, warn};
use protocol::{
    packets::{ConnectionState, PlayerContext},
    serial::{PacketError, PacketRead, PacketWrite},
    types::var_int::VarInt,
};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    sync::broadcast,
};

use crate::states::{
    PacketDispatcher, handshaking::HandshakePacketHandler, status::StatusPacketHandler,
};
pub struct PlayerConnection {
    socket: TcpStream,
    buffer: BytesMut,
    state: ConnectionState,
    shutdown_tx: broadcast::Receiver<()>,
    protocol: i32,
}

impl PlayerConnection {
    pub fn new(socket: TcpStream, shutdown_tx: broadcast::Receiver<()>) -> Self {
        PlayerConnection {
            socket,
            buffer: BytesMut::with_capacity(4096),
            state: ConnectionState::Handshaking,
            shutdown_tx,
            protocol: 0,
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
        let packet_id = VarInt::read(&mut cursor)?.0;

        //info!("received packet with id {}", packet_id);

        match self.state {
            ConnectionState::Handshaking => {
                let mut handler = HandshakePacketHandler::from_id(packet_id, &mut cursor)?;
                handler.dispatch_packet(self).await?;
            }
            ConnectionState::Status => {
                let mut handler = StatusPacketHandler::from_id(packet_id, &mut cursor)?;
                handler.dispatch_packet(self).await?;
            }
            ConnectionState::Login => {
                warn!("login state is not implemented");
            }
            ConnectionState::Play => {
                warn!("play state is not implemented");
            }
        }

        Ok(())
    }
}

#[async_trait]
impl PlayerContext for PlayerConnection {
    fn get_protocol(&self) -> i32 {
        self.protocol
    }

    fn get_state(&self) -> &ConnectionState {
        return &self.state;
    }

    fn set_state(&mut self, state: ConnectionState) {
        self.state = state;
    }

    async fn send_packet(
        &mut self,
        packet: &dyn protocol::serial::PacketWrite,
    ) -> Result<(), PacketError> {
        let mut body = BytesMut::new();

        packet.write(&mut body)?;

        let len = body.len();
        let len = VarInt(len as i32);

        let mut buffer = BytesMut::new();
        len.write(&mut buffer)?;
        buffer.extend_from_slice(&body);

        self.socket.write_all(&buffer).await?;
        self.socket.flush().await?;

        Ok(())
    }

    fn set_protocol(&mut self, protocol: i32) {
        self.protocol = protocol;
    }
}
