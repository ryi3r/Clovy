use crate::core::{reader::Reader, serializing::Serialize, writer::Writer, lists::GMSimpleList, models::language::Language};
use bstr::BString;
use byteorder::WriteBytesExt;
use std::{fmt::Write, io::{Read, Result, Seek}};

#[derive(Default, Clone)]
pub struct ChunkLANG {
    pub unknown1: i32,
    pub language_count: i32,
    pub entry_count: i32,
    pub entry_ids: GMSimpleList<BString>,
    pub languages: GMSimpleList<Language>,
}

impl Serialize for ChunkLANG {
    fn deserialize<R>(reader: &mut Reader<R>) -> Result<Self>
        where R: Read + Seek,
    {
        let mut chunk = Self {
            ..Default::default()
        };

        chunk.unknown1 = reader.read_i32()?;
        chunk.language_count = reader.read_i32()?;
        chunk.entry_count = reader.read_i32()?;

        for _ in 0..chunk.entry_count {
            chunk.entry_ids.push(reader.read_pointer_string()?);
        }
        reader.global_data.lang_entry_count = chunk.entry_count;
        for _ in 0..chunk.language_count {
            chunk.languages.push(Language::deserialize(reader)?);
        }

        Ok(chunk)
    }

    fn serialize<W>(chunk: &Self, writer: &mut Writer<W>) -> Result<()>
        where W: Write + WriteBytesExt + Seek,
    {
        writer.write_i32(chunk.unknown1)?;
        writer.write_i32(chunk.languages.len() as _)?;
        writer.write_i32(chunk.entry_ids.len() as _)?;

        for string in chunk.entry_ids.values.iter() {
            writer.write_pointer_string(string)?;
        }
        for language in chunk.languages.values.iter() {
            Language::serialize(language, writer)?;
        }

        Ok(())
    }
}
