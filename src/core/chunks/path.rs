use crate::core::{reader::Reader, serializing::Serialize, writer::Writer, lists::GMPointerList, models::path::Path};
use byteorder::WriteBytesExt;
use std::{fmt::Write, io::{Read, Seek}};

#[derive(Default, Clone)]
pub struct ChunkPATH {
    pub paths: GMPointerList<Path>,
}

impl Serialize for ChunkPATH {
    fn deserialize<R>(reader: &mut Reader<R>) -> Self
        where R: Read + Seek,
    {
        let mut chunk = Self {
            ..Default::default()
        };

        chunk.paths.deserialize(reader, None, None);

        chunk
    }

    fn serialize<W>(chunk: &Self, writer: &mut Writer<W>)
        where W: Write + WriteBytesExt + Seek,
    {
        chunk.paths.serialize(writer, None, None);
    }
}
