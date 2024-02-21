use crate::core::{reader::Reader, serializing::Serialize, writer::Writer, lists::GMSimpleList, models::language::Language};
use bstr::BString;
use byteorder::WriteBytesExt;
use std::{fmt::Write, io::{Read, Seek}};

#[derive(Default, Clone)]
pub struct ChunkLANG {
    pub unknown1: i32,
    pub language_count: i32,
    pub entry_count: i32,
    pub entry_ids: GMSimpleList<BString>,
    pub languages: GMSimpleList<Language>,
}

impl Serialize for ChunkLANG {
    fn deserialize<R>(reader: &mut Reader<R>) -> Self
        where R: Read + Seek,
    {
        let mut chunk = Self {
            ..Default::default()
        };

        chunk.unknown1 = reader.read_i32().expect("Failed to read unknown1");
        chunk.language_count = reader.read_i32().expect("Failed to read language_count");
        chunk.entry_count = reader.read_i32().expect("Failed to read entry_count");

        for _ in 0..chunk.entry_count {
            chunk.entry_ids.push(reader.read_pointer_string().expect("Failed to read entry ID"));
        }
        reader.global_data.lang_entry_count = chunk.entry_count;
        for _ in 0..chunk.language_count {
            chunk.languages.push(Language::deserialize(reader));
        }

        chunk
    }

    fn serialize<W>(chunk: &Self, writer: &mut Writer<W>)
        where W: Write + WriteBytesExt + Seek,
    {
        writer.write_i32(chunk.unknown1).expect("Failed to write unknown1");
        writer.write_i32(chunk.languages.len() as _).expect("Failed to write language_count");
        writer.write_i32(chunk.entry_ids.len() as _).expect("Failed to write entry_count");

        for string in chunk.entry_ids.values.iter() {
            writer.write_pointer_string(string).expect("Failed to write string");
        }
        for language in chunk.languages.values.iter() {
            Language::serialize(language, writer);
        }
    }
}
