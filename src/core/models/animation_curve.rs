use crate::core::{reader::Reader, serializing::Serialize, writer::Writer, lists::GMSimpleList};
use bitflags::bitflags;
use bstr::BString;
use byteorder::WriteBytesExt;
use std::{fmt::Write, io::{Read, Seek}};

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
    fn deserialize<R>(reader: &mut Reader<R>) -> Self
        where R: Read + Seek,
    {
        let mut chunk = Self {
            ..Default::default()
        };

        chunk.name = reader.read_pointer_string().expect("Failed to read name");
        chunk.graph_type = GraphType::from_bits_retain(reader.read_i32().expect("Failed to read graph_type"));
        chunk.channels.deserialize(reader, None, None);

        chunk
    }

    fn serialize<W>(chunk: &Self, writer: &mut Writer<W>)
        where W: Write + WriteBytesExt + Seek,
    {
        writer.write_pointer_string(&chunk.name).expect("Failed to write name");
        writer.write_i32(chunk.graph_type.bits()).expect("Failed to write graph_type");
        chunk.channels.serialize(writer, None, None);
    }
}

impl Serialize for Channel {
    fn deserialize<R>(reader: &mut Reader<R>) -> Self
        where R: Read + Seek,
    {
        let mut chunk = Self {
            ..Default::default()
        };

        chunk.name = reader.read_pointer_string().expect("Failed to read name");
        chunk.function_type = FunctionType::from_bits_retain(reader.read_i32().expect("Failed to read function type"));
        chunk.iterations = reader.read_u32().expect("Failed to read iterations");
        chunk.points.deserialize(reader, None, None);

        chunk
    }

    fn serialize<W>(chunk: &Self, writer: &mut Writer<W>)
        where W: Write + WriteBytesExt + Seek,
    {
        writer.write_pointer_string(&chunk.name).expect("Failed to write name");
        writer.write_i32(chunk.function_type.bits()).expect("Failed to write function type");
        writer.write_u32(chunk.iterations).expect("Failed to write iterations");
        chunk.points.serialize(writer, None, None);
    }
}

impl Serialize for Point {
    fn deserialize<R>(reader: &mut Reader<R>) -> Self
        where R: Read + Seek,
    {
        let mut chunk = Self {
            ..Default::default()
        };

        chunk.x = reader.read_f32().expect("Failed to read x");
        chunk.value = reader.read_f32().expect("Failed to read value");
        if reader.read_u32().expect("Failed to check version") != 0 {
            reader.version_info.set_version(2, 3, 1, 0);
            reader.seek_relative(-4).expect("Failed to seek back");
        } else {
            if reader.read_u32().expect("Failed to check version") == 0 {
                reader.version_info.set_version(2, 3, 1, 0);
            }
            reader.seek_relative(-8).expect("Failed to seek back");
        }
        if reader.version_info.is_version_at_least(2, 3, 1, 0) {
            let point_x0 = reader.read_f32().expect("Failed to read x0");
            let point_y0 = reader.read_f32().expect("Failed to read y0");
            let point_x1 = reader.read_f32().expect("Failed to read x1");
            let point_y1 = reader.read_f32().expect("Failed to read y1");
            chunk.bezier_points = [point_x0, point_y0, point_x1, point_y1];
        } else {
            reader.seek_relative(4).expect("Failed to seek foward");
        }

        chunk
    }

    fn serialize<W>(chunk: &Self, writer: &mut Writer<W>)
        where W: Write + WriteBytesExt + Seek,
    {
        writer.write_f32(chunk.x).expect("Failed to write x");
        writer.write_f32(chunk.value).expect("Failed to write value");
        if writer.version_info.is_version_at_least(2, 3, 1, 0) {
            writer.write_f32(chunk.bezier_points[0]).expect("Failed to write x0");
            writer.write_f32(chunk.bezier_points[1]).expect("Failed to write y0");
            writer.write_f32(chunk.bezier_points[2]).expect("Failed to write x1");
            writer.write_f32(chunk.bezier_points[3]).expect("Failed to write y1");
        } else {
            writer.write_u32(0).expect("Failed to write value (always 0)");
        }
    }
}
