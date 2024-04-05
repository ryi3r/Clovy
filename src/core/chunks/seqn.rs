use crate::core::{reader::Reader, serializing::Serialize, writer::Writer, lists::GMPointerList, models::sequence::Sequence};
use byteorder::WriteBytesExt;
use std::{fmt::Write, io::{Read, Seek, Result}};

#[derive(Default, Clone)]
pub struct ChunkSEQN {
    pub sequences: GMPointerList<Sequence>,
    pub version: i32,
}

impl Serialize for ChunkSEQN {
    fn deserialize<R>(reader: &mut Reader<R>) -> Result<Self>
        where R: Read + Seek,
    {
        let mut chunk = Self {
            ..Default::default()
        };

        reader.pad_check_byte(4, 0)?;
        chunk.version = reader.read_i32()?;
        chunk.sequences.deserialize(reader, None, None)?;

        Ok(chunk)
    }

    fn serialize<W>(chunk: &Self, writer: &mut Writer<W>) -> Result<()>
        where W: Write + WriteBytesExt + Seek,
    {
        writer.write_i32(chunk.version)?;
        chunk.sequences.serialize(writer, None, None)?;

        Ok(())
    }
}
