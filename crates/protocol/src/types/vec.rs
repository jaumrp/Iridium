use crate::{
    serial::{PacketRead, PacketWrite},
    types::var_int::VarInt,
};

impl<T: PacketRead> PacketRead for Vec<T> {
    fn read<Buffer: bytes::Buf>(buffer: &mut Buffer) -> Result<Self, crate::serial::PacketError> {
        let len = VarInt::read(buffer)?.0;
        let mut items = Vec::with_capacity(len as usize);

        for _ in 0..len {
            items.push(T::read(buffer)?);
        }
        Ok(items)
    }
}

impl<T: PacketWrite> PacketWrite for Vec<T> {
    fn write<Buffer: bytes::BufMut>(
        &self,
        buffer: &mut Buffer,
    ) -> Result<(), crate::serial::PacketError> {
        VarInt(self.len() as i32).write(buffer)?;
        for item in self {
            item.write(buffer)?;
        }
        Ok(())
    }
}
