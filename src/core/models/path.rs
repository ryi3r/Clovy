use crate::core::{reader::Reader, serializing::Serialize, writer::Writer, lists::GMSimpleList};
use bstr::BString;
use byteorder::WriteBytesExt;
use std::{fmt::Write, io::{Read, Seek}};

#[derive(Default, Clone)]
pub struct Path {
    pub name: BString,
    pub smooth: bool,
    pub closed: bool,
    pub precision: u32,
    pub points: GMSimpleList<Point>,
}

impl Serialize for Path {
    fn deserialize<R>(reader: &mut Reader<R>) -> Self
        where R: Read + Seek,
    {
        let mut chunk = Self {
            ..Default::default()
        };

        chunk.name = reader.read_pointer_string().expect("Failed to read name");
        chunk.smooth = reader.read_wide_bool().expect("Failed to read smooth");
        chunk.closed = reader.read_wide_bool().expect("Failed to read closed");
        chunk.precision = reader.read_u32().expect("Failed to read precision");
        chunk.points.deserialize(reader, None, None);

        chunk
    }

    fn serialize<W>(chunk: &Self, writer: &mut Writer<W>)
        where W: Write + WriteBytesExt + Seek,
    {
        writer.write_pointer_string(&chunk.name).expect("Failed to write name");
        writer.write_wide_bool(chunk.smooth).expect("Failed to write smooth");
        writer.write_wide_bool(chunk.closed).expect("Failed to write closed");
        writer.write_u32(chunk.precision).expect("Failed to write precision");
        chunk.points.serialize(writer, None, None);
    }
}

#[derive(Default, Clone)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub speed: f32,
}

impl Serialize for Point {
    fn deserialize<R>(reader: &mut Reader<R>) -> Self
        where R: Read + Seek,
    {
        let mut chunk = Self {
            ..Default::default()
        };

        chunk.x = reader.read_f32().expect("Failed to read x");
        chunk.y = reader.read_f32().expect("Failed to read y");
        chunk.speed = reader.read_f32().expect("Failed to read speed");

        chunk
    }

    fn serialize<W>(chunk: &Self, writer: &mut Writer<W>)
        where W: Write + WriteBytesExt + Seek,
    {
        writer.write_f32(chunk.x).expect("Failed to write x");
        writer.write_f32(chunk.y).expect("Failed to write y");
        writer.write_f32(chunk.speed).expect("Failed to write speed");
    }
}
