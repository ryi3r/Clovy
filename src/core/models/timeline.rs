use crate::core::{reader::Reader, serializing::Serialize, writer::Writer};
use bstr::BString;
use byteorder::WriteBytesExt;
use std::{fmt::Write, io::{Read, Result, Seek}};

#[derive(Default, Clone)]
pub struct Timeline {
    name: BString,
    moments: Vec<(i32, u32)>, // TODO: Change u32 for Object::Event::Action
}

impl Serialize for Timeline {
    fn deserialize<R>(reader: &mut Reader<R>) -> Result<Self>
        where R: Read + Seek,
    {
        let mut chunk = Self {
            ..Default::default()
        };

        chunk.name = reader.read_pointer_string()?;
        for _ in 0..reader.read_u32()? {
            let time = reader.read_i32()?;
            chunk.moments.push((time, reader.read_u32()?));
        }

        Ok(chunk)
    }

    fn serialize<W>(chunk: &Self, writer: &mut Writer<W>) -> Result<()>
        where W: Write + WriteBytesExt + Seek,
    {
        writer.write_pointer_string(&chunk.name)?;
        writer.write_u32(chunk.moments.len() as u32)?;
        for moment in chunk.moments.iter() {
            writer.write_i32(moment.0)?;
            writer.write_pointer::<u32>(0)?; // TODO: Write pointer instead
        }

        Ok(())
    }
}
