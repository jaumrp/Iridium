use bytes::{BufMut, BytesMut};

use crate::{
    serial::{PacketError, PacketRead, PacketWrite},
    types::var_int::VarInt,
};

impl PacketRead for u16 {
    fn read<Buffer: bytes::Buf>(buffer: &mut Buffer) -> Result<Self, crate::serial::PacketError> {
        if buffer.remaining() < 2 {
            return Err(crate::serial::PacketError::Incomplete);
        }
        Ok(buffer.get_u16())
    }
}

impl PacketWrite for u16 {
    fn write(&self, buffer: &mut BytesMut) -> Result<(), crate::serial::PacketError> {
        buffer.put_u16(*self);
        Ok(())
    }
}

impl PacketRead for i64 {
    fn read<Buffer: bytes::Buf>(buffer: &mut Buffer) -> Result<Self, crate::serial::PacketError> {
        if buffer.remaining() < 2 {
            return Err(crate::serial::PacketError::Incomplete);
        }
        Ok(buffer.get_i64())
    }
}

impl PacketWrite for i64 {
    fn write(&self, buffer: &mut BytesMut) -> Result<(), crate::serial::PacketError> {
        buffer.put_i64(*self);
        Ok(())
    }
}

impl PacketRead for String {
    fn read<Buffer: bytes::Buf>(buffer: &mut Buffer) -> Result<Self, crate::serial::PacketError> {
        let len = VarInt::read(buffer)?.0;

        if len > 32767 {
            return Err(crate::serial::PacketError::StringTooLong);
        }

        let len = len as usize;

        if buffer.remaining() < len {
            return Err(crate::serial::PacketError::Incomplete);
        }

        let mut bytes = vec![0u8; len];
        buffer.copy_to_slice(&mut bytes);

        String::from_utf8(bytes).map_err(PacketError::Utf8)
    }
}

impl PacketWrite for String {
    fn write(&self, buffer: &mut BytesMut) -> Result<(), PacketError> {
        VarInt(self.len() as i32).write(buffer)?;
        buffer.put_slice(self.as_bytes());
        Ok(())
    }
}
