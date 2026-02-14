use crate::serial::{PacketRead, PacketWrite};

pub struct Property {
    pub name: String,
    pub value: String,
    pub signature: Option<String>,
}

impl PacketRead for Property {
    fn read<Buffer: bytes::Buf>(buffer: &mut Buffer) -> Result<Self, crate::serial::PacketError> {
        Ok(Property {
            name: String::read(buffer)?,
            value: String::read(buffer)?,
            signature: Option::<String>::read(buffer)?,
        })
    }
}

impl PacketWrite for Property {
    fn write(&self, buffer: &mut bytes::BytesMut) -> Result<(), crate::serial::PacketError> {
        self.name.write(buffer)?;
        self.value.write(buffer)?;
        self.signature.write(buffer)?;
        Ok(())
    }
}
