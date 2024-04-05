use crate::core::{reader::Reader, serializing::Serialize, writer::Writer, lists::GMSimpleList};
use bstr::BString;
use byteorder::WriteBytesExt;
use std::{fmt::Write, io::{Read, Result, Seek}};

#[derive(Default, Clone)]
pub struct Language {
    name: BString,
    region: BString,
    entries: GMSimpleList<BString>,
}

impl Serialize for Language {
    fn deserialize<R>(reader: &mut Reader<R>) -> Result<Self>
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

        Ok(chunk)
    }

    fn serialize<W>(chunk: &Self, writer: &mut Writer<W>) -> Result<()>
        where W: Write + WriteBytesExt + Seek,
    {
        writer.write_pointer_string(&chunk.name)?;
        writer.write_pointer_string(&chunk.region)?;
        for string in chunk.entries.values.iter() {
            writer.write_pointer_string(string)?;
        }

        Ok(())
    }
}
