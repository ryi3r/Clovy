use crate::core::{reader::Reader, serializing::Serialize, writer::Writer};
use byteorder::WriteBytesExt;
use std::{fmt::Write, io::{Read, Result, Seek}};

#[derive(Default, Clone)]
pub struct ChunkSHDR {
    // Nothing.
}

impl Serialize for ChunkSHDR {
    fn deserialize<R>(_reader: &mut Reader<R>) -> Result<Self>
        where R: Read + Seek,
    {
        /*let chunk = Self {
        };*/

        //todo!("Not implemented");

        Ok(Self {})
    }

    fn serialize<W>(_chunk: &Self, _writer: &mut Writer<W>) -> Result<()>
        where W: Write + WriteBytesExt + Seek,
    {
        todo!("Not implemented");
    }
}
