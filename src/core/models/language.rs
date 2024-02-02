use crate::core::{reader::Reader, serializing::Serialize, writer::Writer, lists::GMSimpleList};
use bstr::BString;
use byteorder::WriteBytesExt;
use std::{fmt::Write, io::{Read, Seek}};

#[derive(Default, Clone)]
pub struct Language {
    name: BString,
    region: BString,
    entries: GMSimpleList<BString>,
}

impl Serialize for Language {
    fn deserialize<R>(reader: &mut Reader<R>) -> Self
        where R: Read + Seek,
    {
        let mut chunk = Self {
            ..Default::default()
        };

        chunk.name = reader.read_pointer_string().expect("Failed to read name");
        chunk.region = reader.read_pointer_string().expect("Failed to read region");
        for _ in 0..reader.global_data.lang_entry_count {
            chunk.entries.push(reader.read_pointer_string().expect("Failed to read string"));
        }

        chunk
    }

    fn serialize<W>(chunk: &Self, writer: &mut Writer<W>)
        where W: Write + WriteBytesExt + Seek,
    {
        writer.write_pointer_string(&chunk.name).expect("Failed to write name");
        writer.write_pointer_string(&chunk.region).expect("Failed to write region");
        for string in chunk.entries.values.iter() {
            writer.write_pointer_string(string).expect("Failed to write string");
        }
    }
}
