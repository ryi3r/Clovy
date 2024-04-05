use crate::core::{reader::Reader, serializing::Serialize, writer::Writer, lists::GMPointerList, models::filter_effect::FilterEffect};
use byteorder::WriteBytesExt;
use std::{fmt::Write, io::{Read, Result, Seek}};

#[derive(Default, Clone)]
pub struct ChunkFEDS {
    pub filter_effect: GMPointerList<FilterEffect>,
    pub version: i32,
}

impl Serialize for ChunkFEDS {
    fn deserialize<R>(reader: &mut Reader<R>) -> Result<Self>
        where R: Read + Seek,
    {
        let mut chunk = Self {
            ..Default::default()
        };

        reader.pad(4)?;
        chunk.version = reader.read_i32()?;
        chunk.filter_effect.deserialize(reader, None, None)?;

        Ok(chunk)
    }

    fn serialize<W>(chunk: &Self, writer: &mut Writer<W>) -> Result<()>
        where W: Write + WriteBytesExt + Seek,
    {
        writer.pad(4)?;
        writer.write_i32(chunk.version)?;
        chunk.filter_effect.serialize(writer, None, None)?;

        Ok(())
    }
}
