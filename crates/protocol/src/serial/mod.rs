use async_trait::async_trait;
use bytes::{Buf, BytesMut};

use crate::packets::PlayerContext;

#[derive(thiserror::Error, Debug)]
pub enum PacketError {
    #[error("Incomplete packet")]
    Incomplete,

    #[error("String too long")]
    StringTooLong,

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("UTF-8 error: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),

    #[error("Unknown packet")]
    UnknownPacket,

    #[error("Invalid data")]
    InvalidData,
}

pub trait PacketWrite: Send + Sync {
    fn write(&self, buffer: &mut BytesMut) -> Result<(), PacketError>;
}

pub trait PacketRead {
    fn read<Buffer: Buf>(buffer: &mut Buffer) -> Result<Self, PacketError>
    where
        Self: Sized;
}

#[async_trait]
pub trait PacketHandler {
    async fn handle<Context: PlayerContext>(
        &mut self,
        ctx: &mut Context,
    ) -> Result<(), PacketError>;
}
