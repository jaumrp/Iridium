use bytes::{BufMut, BytesMut};

use crate::serial::{PacketError, PacketRead, PacketWrite};

pub type VarIntType = i32;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VarInt(pub VarIntType);

impl PacketWrite for VarInt {
    fn write(&self, buffer: &mut BytesMut) -> Result<(), crate::serial::PacketError> {
        let mut x = self.0 as u32;

        loop {
            let mut temp = (x & 0x7F) as u8;
            x >>= 7;
            if x != 0 {
                temp |= 0x80;
            }
            buffer.put_u8(temp);
            if x == 0 {
                break;
            }
        }

        Ok(())
    }
}

impl PacketRead for VarInt {
    fn read<Buffer: bytes::Buf>(buffer: &mut Buffer) -> Result<Self, crate::serial::PacketError> {
        let mut num = 0;
        let mut shift = 0;

        loop {
            if !buffer.has_remaining() {
                return Err(crate::serial::PacketError::Incomplete);
            }
            let byte = buffer.get_u8();
            num |= ((byte & 0x7F) as i32) << shift;
            if (byte & 0x80) == 0 {
                return Ok(VarInt(num));
            }
            shift += 7;
            if shift >= 32 {
                return Err(PacketError::Io(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "VarInt too big",
                )));
            }
        }
    }
}
