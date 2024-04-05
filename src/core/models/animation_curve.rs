use crate::core::{reader::Reader, serializing::Serialize, writer::Writer, lists::GMSimpleList};
use bitflags::bitflags;
use bstr::BString;
use byteorder::WriteBytesExt;
use std::{fmt::Write, io::{Read, Result, Seek}};

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct GraphType: i32 {
        const Unknown1 = 0;
        const Unknown2 = 1;
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct FunctionType: i32 {
        const Linear = 0;
        const Smooth = 1;
        const Bezier = 2;
    }
}

impl Default for GraphType {
    fn default() -> Self {
        Self::Unknown1
    }
}

impl Default for FunctionType {
    fn default() -> Self {
        Self::Linear
    }
}

#[derive(Default, Clone)]
pub struct AnimationCurve {
    pub name: BString,
    pub graph_type: GraphType,
    pub channels: GMSimpleList<Channel>,
}

#[derive(Default, Clone)]
pub struct Channel {
    pub name: BString,
    pub function_type: FunctionType,
    pub iterations: u32,
    pub points: GMSimpleList<Point>,
}

#[derive(Default, Clone)]
pub struct Point {
    pub x: f32,
    pub value: f32,
    pub bezier_points: [f32; 4], // [x0, y0, x1, y1]
}

impl Serialize for AnimationCurve {
    fn deserialize<R>(reader: &mut Reader<R>) -> Result<Self>
        where R: Read + Seek,
    {
        let mut chunk = Self {
            ..Default::default()
        };

        chunk.name = reader.read_pointer_string()?;
        chunk.graph_type = GraphType::from_bits_retain(reader.read_i32()?);
        chunk.channels.deserialize(reader, None, None)?;

        Ok(chunk)
    }

    fn serialize<W>(chunk: &Self, writer: &mut Writer<W>) -> Result<()>
        where W: Write + WriteBytesExt + Seek,
    {
        writer.write_pointer_string(&chunk.name)?;
        writer.write_i32(chunk.graph_type.bits())?;
        chunk.channels.serialize(writer, None, None)?;

        Ok(())
    }
}

impl Serialize for Channel {
    fn deserialize<R>(reader: &mut Reader<R>) -> Result<Self>
        where R: Read + Seek,
    {
        let mut chunk = Self {
            ..Default::default()
        };

        chunk.name = reader.read_pointer_string()?;
        chunk.function_type = FunctionType::from_bits_retain(reader.read_i32()?);
        chunk.iterations = reader.read_u32()?;
        chunk.points.deserialize(reader, None, None)?;

        Ok(chunk)
    }

    fn serialize<W>(chunk: &Self, writer: &mut Writer<W>) -> Result<()>
        where W: Write + WriteBytesExt + Seek,
    {
        writer.write_pointer_string(&chunk.name)?;
        writer.write_i32(chunk.function_type.bits())?;
        writer.write_u32(chunk.iterations)?;
        chunk.points.serialize(writer, None, None)?;

        Ok(())
    }
}

impl Serialize for Point {
    fn deserialize<R>(reader: &mut Reader<R>) -> Result<Self>
        where R: Read + Seek,
    {
        let mut chunk = Self {
            ..Default::default()
        };

        chunk.x = reader.read_f32()?;
        chunk.value = reader.read_f32()?;
        if reader.version_info.is_version_at_least(2, 3, 1, 0) {
            let point_x0 = reader.read_f32()?;
            let point_y0 = reader.read_f32()?;
            let point_x1 = reader.read_f32()?;
            let point_y1 = reader.read_f32()?;
            chunk.bezier_points = [point_x0, point_y0, point_x1, point_y1];
        } else {
            reader.seek_relative(4)?;
        }

        Ok(chunk)
    }

    fn serialize<W>(chunk: &Self, writer: &mut Writer<W>) -> Result<()>
        where W: Write + WriteBytesExt + Seek,
    {
        writer.write_f32(chunk.x)?;
        writer.write_f32(chunk.value)?;
        if writer.version_info.is_version_at_least(2, 3, 1, 0) {
            writer.write_f32(chunk.bezier_points[0])?;
            writer.write_f32(chunk.bezier_points[1])?;
            writer.write_f32(chunk.bezier_points[2])?;
            writer.write_f32(chunk.bezier_points[3])?;
        } else {
            writer.write_u32(0)?;
        }

        Ok(())
    }
}
