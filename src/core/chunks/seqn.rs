use crate::core::{reader::Reader, serializing::Serialize, writer::Writer, lists::GMPointerList, models::sequence::Sequence};
use byteorder::WriteBytesExt;
use std::{fmt::Write, io::{Read, Seek}};

#[derive(Default, Clone)]
pub struct ChunkSEQN {
    pub sequences: GMPointerList<Sequence>,
    pub version: i32,
}

impl Serialize for ChunkSEQN {
    fn deserialize<R>(reader: &mut Reader<R>) -> Self
        where R: Read + Seek,
    {
        let mut chunk = Self {
            ..Default::default()
        };

        reader.pad_check_byte(4, 0).expect("Failed to pad");
        chunk.version = reader.read_i32().expect("Failed to read version");
        chunk.sequences.deserialize(reader, None, None);

        chunk
    }

    fn serialize<W>(chunk: &Self, writer: &mut Writer<W>)
        where W: Write + WriteBytesExt + Seek,
    {
        writer.write_i32(chunk.version).expect("Failed to write version");
        chunk.sequences.serialize(writer, None, None);
    }
}
