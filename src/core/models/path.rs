use crate::core::{reader::Reader, serializing::Serialize, writer::Writer, lists::GMSimpleList};
use bstr::BString;
use byteorder::WriteBytesExt;
use std::{fmt::Write, io::{Read, Result, Seek}};

#[derive(Default, Clone)]
pub struct Path {
    pub name: BString,
    pub smooth: bool,
    pub closed: bool,
    pub precision: u32,
    pub points: GMSimpleList<Point>,
}

impl Serialize for Path {
    fn deserialize<R>(reader: &mut Reader<R>) -> Result<Self>
        where R: Read + Seek,
    {
        let mut chunk = Self {
            ..Default::default()
        };

        chunk.name = reader.read_pointer_string()?;
        chunk.smooth = reader.read_wide_bool()?;
        chunk.closed = reader.read_wide_bool()?;
        chunk.precision = reader.read_u32()?;
        chunk.points.deserialize(reader, None, None)?;

        Ok(chunk)
    }

    fn serialize<W>(chunk: &Self, writer: &mut Writer<W>) -> Result<()>
        where W: Write + WriteBytesExt + Seek,
    {
        writer.write_pointer_string(&chunk.name)?;
        writer.write_wide_bool(chunk.smooth)?;
        writer.write_wide_bool(chunk.closed)?;
        writer.write_u32(chunk.precision)?;
        chunk.points.serialize(writer, None, None)?;

        Ok(())
    }
}

#[derive(Default, Clone)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub speed: f32,
}

impl Serialize for Point {
    fn deserialize<R>(reader: &mut Reader<R>) -> Result<Self>
        where R: Read + Seek,
    {
        let mut chunk = Self {
            ..Default::default()
        };

        chunk.x = reader.read_f32()?;
        chunk.y = reader.read_f32()?;
        chunk.speed = reader.read_f32()?;

        Ok(chunk)
    }

    fn serialize<W>(chunk: &Self, writer: &mut Writer<W>) -> Result<()>
        where W: Write + WriteBytesExt + Seek,
    {
        writer.write_f32(chunk.x)?;
        writer.write_f32(chunk.y)?;
        writer.write_f32(chunk.speed)?;

        Ok(())
    }
}
