use crate::core::{reader::Reader, serializing::Serialize, writer::Writer};
use byteorder::WriteBytesExt;
use std::{fmt::Write, io::{Read, Result, Seek}};

#[derive(Default, Clone)]
pub struct ChunkGLOB {
    pub global_init_entries: Vec<i32>,
}

impl Serialize for ChunkGLOB {
    fn deserialize<R>(reader: &mut Reader<R>) -> Result<Self>
        where R: Read + Seek,
    {
        let mut chunk = Self {
            ..Default::default()
        };

        for _ in 0..reader.read_u32()? {
            chunk.global_init_entries.push(reader.read_i32()?);
        }

        Ok(chunk)
    }

    fn serialize<W>(chunk: &Self, writer: &mut Writer<W>) -> Result<()>
        where W: Write + WriteBytesExt + Seek,
    {
        writer.write_u32(chunk.global_init_entries.len() as _)?;
        for global_init_entry in chunk.global_init_entries.iter() {
            writer.write_i32(*global_init_entry)?;
        }

        Ok(())
    }
}
