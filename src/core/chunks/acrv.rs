use crate::core::{reader::Reader, serializing::Serialize, writer::Writer, lists::GMPointerList, models::animation_curve::AnimationCurve};
use byteorder::WriteBytesExt;
use std::{fmt::Write, io::{Read, Result, Seek}};

#[derive(Default, Clone)]
pub struct ChunkACRV {
    pub animation_curves: GMPointerList<AnimationCurve>,
    pub version: i32,
}

impl Serialize for ChunkACRV {
    fn deserialize<R>(reader: &mut Reader<R>) -> Result<Self>
        where R: Read + Seek,
    {
        let mut chunk = Self {
            ..Default::default()
        };

        chunk.version = reader.read_i32()?;
        chunk.animation_curves.deserialize(reader, None, None)?;

        Ok(chunk)
    }

    fn serialize<W>(chunk: &Self, writer: &mut Writer<W>) -> Result<()>
        where W: Write + WriteBytesExt + Seek,
    {
        writer.write_i32(chunk.version)?;
        chunk.animation_curves.serialize(writer, None, None)?;

        Ok(())
    }
}
