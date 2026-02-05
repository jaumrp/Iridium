use protocol::packets::server::configuration::handshake::HandshakePacket;

pub enum HandshakingState {
    Handshake(HandshakePacket),
}
