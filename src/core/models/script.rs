use crate::core::{reader::Reader, serializing::Serialize, writer::Writer};
use bstr::BString;
use byteorder::WriteBytesExt;
use std::{fmt::Write, io::{Read, Result, Seek}};

#[derive(Default, Clone)]
pub struct Script {
    pub name: BString,
    pub code_id: i32,
    pub constructor: bool,
}

impl Serialize for Script {
    fn deserialize<R>(reader: &mut Reader<R>) -> Result<Self>
        where R: Read + Seek,
    {
        let mut chunk = Self {
            ..Default::default()
        };

        chunk.name = reader.read_pointer_string()?;
        chunk.code_id = reader.read_i32()?;
        if chunk.code_id < -1 {
            chunk.constructor = true;
            chunk.code_id = (chunk.code_id as u32 & 0x7fffffff) as i32;
        }

        Ok(chunk)
    }

    fn serialize<W>(chunk: &Self, writer: &mut Writer<W>) -> Result<()>
        where W: Write + WriteBytesExt + Seek,
    {
        writer.write_pointer_string(&chunk.name)?;
        if chunk.constructor {
            writer.write_u32(chunk.code_id as u32 | 0x80000000)?;
        } else {
            writer.write_i32(chunk.code_id)?;
        }

        Ok(())
    }
}
