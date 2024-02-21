use crate::core::{reader::Reader, serializing::Serialize, writer::Writer};
use bstr::BString;
use byteorder::WriteBytesExt;
use std::{fmt::Write, io::{Read, Seek}};

#[derive(Default, Clone)]
pub struct Script {
    pub name: BString,
    pub code_id: i32,
    pub constructor: bool,
}

impl Serialize for Script {
    fn deserialize<R>(reader: &mut Reader<R>) -> Self
        where R: Read + Seek,
    {
        let mut chunk = Self {
            ..Default::default()
        };

        chunk.name = reader.read_pointer_string().expect("Failed to read name");
        chunk.code_id = reader.read_i32().expect("Failed to read code_id");
        if chunk.code_id < -1 {
            chunk.constructor = true;
            chunk.code_id = (chunk.code_id as u32 & 2147483647) as i32;
        }

        chunk
    }

    fn serialize<W>(chunk: &Self, writer: &mut Writer<W>)
        where W: Write + WriteBytesExt + Seek,
    {
        writer.write_pointer_string(&chunk.name).expect("Failed to write name");
        if chunk.constructor {
            writer.write_u32(chunk.code_id as u32 | 2147483648).expect("Failed to write code id");
        } else {
            writer.write_i32(chunk.code_id).expect("Failed to write code id");
        }
    }
}
