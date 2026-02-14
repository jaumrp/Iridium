use bytes::{BufMut, BytesMut, buf};

use crate::{
    serial::{PacketError, PacketRead, PacketWrite},
    types::var_int::VarInt,
};

macro_rules! impl_primitive {
    ($type:ty, $size:expr, $read_method:ident, $write_method:ident) => {
        impl PacketWrite for $type {
            fn write(&self, buffer: &mut BytesMut) -> Result<(), crate::serial::PacketError> {
                if buffer.remaining_mut() < $size {
                    return Err(crate::serial::PacketError::Incomplete);
                }
                Ok(buffer.$write_method(*self))
            }
        }

        impl PacketRead for $type {
            fn read<Buffer: bytes::Buf>(
                buffer: &mut Buffer,
            ) -> Result<Self, crate::serial::PacketError> {
                if buffer.remaining() < $size {
                    return Err(crate::serial::PacketError::Incomplete);
                }
                Ok(buffer.$read_method())
            }
        }
    };
}

impl_primitive!(u8, 1, get_u8, put_u8);
impl_primitive!(u16, 2, get_u16, put_u16);
impl_primitive!(u32, 4, get_u32, put_u32);
impl_primitive!(u64, 8, get_u64, put_u64);
impl_primitive!(i8, 1, get_i8, put_i8);
impl_primitive!(i16, 2, get_i16, put_i16);
impl_primitive!(i32, 4, get_i32, put_i32);
impl_primitive!(i64, 8, get_i64, put_i64);

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

impl PacketRead for uuid::Uuid {
    fn read<Buffer: bytes::Buf>(buffer: &mut Buffer) -> Result<Self, crate::serial::PacketError> {
        if buffer.remaining() < 16 {
            return Err(crate::serial::PacketError::Incomplete);
        }
        let mut bytes = [0u8; 16];
        buffer.copy_to_slice(&mut bytes);
        Ok(uuid::Uuid::from_bytes(bytes))
    }
}

impl PacketWrite for uuid::Uuid {
    fn write(&self, buffer: &mut BytesMut) -> Result<(), PacketError> {
        buffer.put_slice(self.as_bytes());
        Ok(())
    }
}

impl PacketWrite for bool {
    fn write(&self, buffer: &mut BytesMut) -> Result<(), PacketError> {
        buffer.put_u8(if *self { 1 } else { 0 });
        Ok(())
    }
}

impl PacketRead for bool {
    fn read<Buffer: bytes::Buf>(buffer: &mut Buffer) -> Result<Self, crate::serial::PacketError> {
        if buffer.remaining() < 1 {
            return Err(crate::serial::PacketError::Incomplete);
        }
        Ok(buffer.get_u8() != 0)
    }
}

impl<T: PacketRead> PacketRead for Option<T> {
    fn read<Buffer: buf::Buf>(buffer: &mut Buffer) -> Result<Self, PacketError> {
        let has_value = bool::read(buffer)?;
        if has_value {
            let value = T::read(buffer)?;
            Ok(Some(value))
        } else {
            Ok(None)
        }
    }
}
impl<T: PacketWrite> PacketWrite for Option<T> {
    fn write(&self, buffer: &mut BytesMut) -> Result<(), PacketError> {
        match self {
            Some(value) => {
                true.write(buffer)?;
                value.write(buffer)?;
            }
            None => false.write(buffer)?,
        }
        Ok(())
    }
}
