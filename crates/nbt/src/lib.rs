use std::io::{Error, ErrorKind, Read, Result, Write};

use ahash::AHashMap;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

pub type Compound = AHashMap<String, Value>;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    String(String),
    List(Vec<Value>),
    Compound(Compound),
    ByteArray(Vec<u8>),
    IntArray(Vec<i32>),
    LongArray(Vec<i64>),
}

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Copy, Eq, PartialOrd, Ord)]
pub enum TagId {
    End = 0,
    Byte = 1,
    Short = 2,
    Int = 3,
    Long = 4,
    Float = 5,
    Double = 6,
    ByteArray = 7,
    String = 8,
    List = 9,
    Compound = 10,
    IntArray = 11,
    LongArray = 12,
}

impl From<u8> for TagId {
    fn from(value: u8) -> Self {
        match value {
            1 => TagId::Byte,
            2 => TagId::Short,
            3 => TagId::Int,
            4 => TagId::Long,
            5 => TagId::Float,
            6 => TagId::Double,
            7 => TagId::ByteArray,
            8 => TagId::String,
            9 => TagId::List,
            10 => TagId::Compound,
            11 => TagId::IntArray,
            12 => TagId::LongArray,
            _ => TagId::End,
        }
    }
}

impl Value {
    pub fn id(&self) -> TagId {
        match self {
            Value::Byte(_) => TagId::Byte,
            Value::Short(_) => TagId::Short,
            Value::Int(_) => TagId::Int,
            Value::Long(_) => TagId::Long,
            Value::Float(_) => TagId::Float,
            Value::Double(_) => TagId::Double,
            Value::String(_) => TagId::String,
            Value::List(_) => TagId::List,
            Value::Compound(_) => TagId::Compound,
            Value::ByteArray(_) => TagId::ByteArray,
            Value::IntArray(_) => TagId::IntArray,
            Value::LongArray(_) => TagId::LongArray,
        }
    }

    pub fn new_compound() -> Self {
        Value::Compound(AHashMap::new())
    }
}

impl Value {
    pub fn read_unnamed<R: Read>(reader: &mut R, type_id: TagId) -> Result<Self> {
        match type_id {
            TagId::Byte => Ok(Value::Byte(reader.read_i8()?)),
            TagId::Short => Ok(Value::Short(reader.read_i16::<BigEndian>()?)),
            TagId::Int => Ok(Value::Int(reader.read_i32::<BigEndian>()?)),
            TagId::Long => Ok(Value::Long(reader.read_i64::<BigEndian>()?)),
            TagId::Float => Ok(Value::Float(reader.read_f32::<BigEndian>()?)),
            TagId::Double => Ok(Value::Double(reader.read_f64::<BigEndian>()?)),

            TagId::ByteArray => {
                let len = reader.read_i32::<BigEndian>()?;
                let mut bytes = vec![0u8; len as usize];
                reader.read_exact(&mut bytes)?;
                let bytes = unsafe { std::mem::transmute(bytes) };
                Ok(Value::ByteArray(bytes))
            }

            TagId::String => {
                let len = reader.read_u16::<BigEndian>()?;
                let mut bytes = vec![0u8; len as usize];
                reader.read_exact(&mut bytes)?;
                let bytes =
                    String::from_utf8(bytes).map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
                Ok(Value::String(bytes))
            }

            TagId::List => {
                let element_id = TagId::from(reader.read_u8()?);
                let len = reader.read_i32::<BigEndian>()?;
                let mut values = Vec::with_capacity(len as usize);
                if len > 0 && element_id != TagId::End {
                    for _ in 0..len {
                        values.push(Value::read_unnamed(reader, element_id)?);
                    }
                }
                Ok(Value::List(values))
            }

            TagId::Compound => {
                let mut map = AHashMap::new();

                loop {
                    let tag_id = reader.read_u8()?;
                    if tag_id == 0 {
                        break;
                    }
                    let tag_id = TagId::from(tag_id);
                    let name_len = reader.read_u16::<BigEndian>()?;

                    let mut name_buf = vec![0u8; name_len as usize];
                    reader.read_exact(&mut name_buf)?;
                    let name = String::from_utf8(name_buf)
                        .map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
                    let value = Value::read_unnamed(reader, tag_id)?;
                    map.insert(name, value);
                }

                Ok(Value::Compound(map))
            }

            TagId::IntArray => {
                let len = reader.read_i32::<BigEndian>()?;
                let mut values = Vec::with_capacity(len as usize);
                for _ in 0..len {
                    values.push(reader.read_i32::<BigEndian>()?);
                }
                Ok(Value::IntArray(values))
            }

            TagId::LongArray => {
                let len = reader.read_i32::<BigEndian>()?;
                let mut values = Vec::with_capacity(len as usize);
                for _ in 0..len {
                    values.push(reader.read_i64::<BigEndian>()?);
                }
                Ok(Value::LongArray(values))
            }

            TagId::End => Err(Error::new(ErrorKind::InvalidData, "Unexpected end tag")),
        }
    }
}

impl Value {
    pub fn write_unnamed<W: Write>(&self, writer: &mut W) -> Result<()> {
        match self {
            Value::Byte(v) => writer.write_i8(*v)?,
            Value::Short(v) => writer.write_i16::<BigEndian>(*v)?,
            Value::Int(v) => writer.write_i32::<BigEndian>(*v)?,
            Value::Long(v) => writer.write_i64::<BigEndian>(*v)?,
            Value::Float(v) => writer.write_f32::<BigEndian>(*v)?,
            Value::Double(v) => writer.write_f64::<BigEndian>(*v)?,
            Value::String(v) => {
                writer.write_u16::<BigEndian>(v.len() as u16)?;
                writer.write_all(v.as_bytes())?;
            }
            Value::ByteArray(v) => {
                writer.write_i32::<BigEndian>(v.len() as i32)?;
                writer.write_all(v)?;
            }
            Value::IntArray(v) => {
                writer.write_i32::<BigEndian>(v.len() as i32)?;
                for &value in v {
                    writer.write_i32::<BigEndian>(value)?;
                }
            }
            Value::LongArray(v) => {
                writer.write_i32::<BigEndian>(v.len() as i32)?;
                for &value in v {
                    writer.write_i64::<BigEndian>(value)?;
                }
            }
            Value::Compound(map) => {
                for (name, value) in map {
                    writer.write_u8(TagId::Compound as u8)?;
                    writer.write_u16::<BigEndian>(name.len() as u16)?;
                    writer.write_all(name.as_bytes())?;
                    value.write_unnamed(writer)?;
                }
                writer.write_u8(TagId::End as u8)?;
            }
            Value::List(list) => {
                if list.is_empty() {
                    writer.write_i8(TagId::End as i8)?;
                    writer.write_i32::<BigEndian>(0)?
                } else {
                    let type_id = list[0].id();
                    writer.write_i8(type_id as i8)?;
                    writer.write_i32::<BigEndian>(list.len() as i32)?;
                    for value in list {
                        value.write_unnamed(writer)?;
                    }
                }
            }
        }
        Ok(())
    }
}
