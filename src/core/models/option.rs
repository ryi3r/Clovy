use std::{fmt::Write, io::{Read, Result, Seek}};
use bstr::BString;
use byteorder::WriteBytesExt;
use crate::core::{reader::Reader, serializing::Serialize, writer::Writer};

#[derive(Default, Clone)]
pub struct Constant {
    pub name: BString,
    pub value: BString,
}

impl Serialize for Constant {
    fn deserialize<R>(reader: &mut Reader<R>) -> Result<Self>
        where R: Read + Seek,
    {
        let mut chunk = Self {
            ..Default::default()
        };

        chunk.name = reader.read_pointer_string()?;
        chunk.value = reader.read_pointer_string()?;

        Ok(chunk)
    }
    
    fn serialize<W>(chunk: &Self, writer: &mut Writer<W>) -> Result<()>
        where W: Write + WriteBytesExt + Seek,
    {
        writer.write_pointer_string(&chunk.name)?;
        writer.write_pointer_string(&chunk.value)?;

        Ok(())
    }
}
