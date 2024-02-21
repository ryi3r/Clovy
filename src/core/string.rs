use crate::core::{reader::Reader, serializing::Serialize, writer::Writer};
use bstr::BString;
use byteorder::WriteBytesExt;
use std::{fmt::Write, io::{Read, Seek}};

impl Serialize for BString {
    fn deserialize<R>(reader: &mut Reader<R>) -> Self
        where R: Read + Seek,
    {
        reader.read_pointer_string().expect("Failed to read string")
    }

    fn serialize<W>(chunk: &Self, writer: &mut Writer<W>)
        where W: Write + WriteBytesExt + Seek,
    {
        writer.write_pointer_string(chunk).expect("Failed to write string");
    }
}
