use std::{io::Cursor, sync::Arc};

use bytes::{Buf, BytesMut};
use events::EventBus;
use log::error;
use protocol::{
    ConnectionState,
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
    event_bus: Arc<EventBus>,
}

impl PlayerConnection {
    pub fn new(
        socket: TcpStream,
        shutdown_tx: broadcast::Receiver<()>,
        event_bus: Arc<EventBus>,
    ) -> Self {
        PlayerConnection {
            socket,
            buffer: BytesMut::with_capacity(4096),
            state: ConnectionState::Handshaking,
            shutdown_tx,
            protocol: 0,
            event_bus,
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
                return Err(PacketError::NotImplemented("login".to_string()));
            }
            ConnectionState::Play => {
                return Err(PacketError::NotImplemented("play".to_string()));
            }
        }

        Ok(())
    }
}

impl PlayerConnection {
    pub fn get_protocol(&self) -> i32 {
        self.protocol
    }

    pub fn get_state(&self) -> &ConnectionState {
        return &self.state;
    }

    pub fn set_state(&mut self, state: ConnectionState) {
        self.state = state;
    }

    pub fn set_protocol(&mut self, protocol: i32) {
        self.protocol = protocol;
    }

    pub fn event_bus(&self) -> &EventBus {
        &self.event_bus
    }
}

impl PlayerConnection {
    pub async fn send_packet(
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
}
