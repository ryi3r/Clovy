use crate::core::{reader::Reader, serializing::Serialize, writer::Writer, models::sound::Sound, lists::GMPointerList};
use byteorder::WriteBytesExt;
use std::{fmt::Write, io::{Read, Seek}};

#[derive(Default, Clone)]
pub struct ChunkSOND {
    pub sounds: GMPointerList<Sound>,
}

impl Serialize for ChunkSOND {
    fn deserialize<R>(reader: &mut Reader<R>) -> Self
        where R: Read + Seek,
    {
        let mut chunk = Self {
            ..Default::default()
        };

        chunk.sounds.deserialize(reader, None, None);

        chunk
    }

    fn serialize<W>(chunk: &Self, writer: &mut Writer<W>)
        where W: Write + WriteBytesExt + Seek,
    {
        chunk.sounds.serialize(writer, None, None);
    }
}
