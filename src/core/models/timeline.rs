use crate::core::{reader::Reader, serializing::Serialize, writer::Writer};
use bstr::BString;
use byteorder::WriteBytesExt;
use std::{fmt::Write, io::{Read, Seek}};

#[derive(Default, Clone)]
pub struct Timeline {
    name: BString,
    moments: Vec<(i32, u32)>, // TODO: Change u32 for Object::Event::Action
}

impl Serialize for Timeline {
    fn deserialize<R>(reader: &mut Reader<R>) -> Self
        where R: Read + Seek,
    {
        let mut chunk = Self {
            ..Default::default()
        };

        chunk.name = reader.read_pointer_string().expect("Failed to read name");
        for _ in 0..reader.read_u32().expect("Failed to read count") {
            let time = reader.read_i32().expect("Failed to read time");
            chunk.moments.push((time, reader.read_u32().expect("Failed to read action")));
        }

        chunk
    }

    fn serialize<W>(chunk: &Self, writer: &mut Writer<W>)
        where W: Write + WriteBytesExt + Seek,
    {
        writer.write_pointer_string(&chunk.name).expect("Failed to write name");
        writer.write_u32(chunk.moments.len() as u32).expect("Failed to write count");
        for moment in chunk.moments.iter() {
            writer.write_i32(moment.0).expect("Failed to write time");
            writer.write_pointer::<u32>(0).expect("Failed to write action"); // TODO: Write pointer instead
        }
    }
}
