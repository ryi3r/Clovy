use crate::core::{reader::Reader, serializing::Serialize, writer::Writer, lists::GMPointerList, models::object::Object};
use byteorder::WriteBytesExt;
use std::{fmt::Write, io::{Read, Seek}};

#[derive(Default, Clone)]
pub struct ChunkOBJT {
    pub objects: GMPointerList<Object>,
}

impl Serialize for ChunkOBJT {
    fn deserialize<R>(reader: &mut Reader<R>) -> Self
        where R: Read + Seek,
    {
        let mut chunk = Self {
            ..Default::default()
        };

        chunk.objects.deserialize(reader, None, None);

        chunk
    }

    fn serialize<W>(chunk: &Self, writer: &mut Writer<W>)
        where W: Write + WriteBytesExt + Seek,
    {
        chunk.objects.serialize(writer, None, None);
    }
}
