use crate::core::{reader::Reader, serializing::Serialize, writer::Writer};
use byteorder::WriteBytesExt;
use std::{fmt::Write, io::{Read, Seek}};

#[derive(Default, Clone)]
pub struct DummyData {
    pub dummy_value: i8,
}

impl Serialize for DummyData {
    fn deserialize<R>(_reader: &mut Reader<R>) -> Self
        where R: Read + Seek,
    {
        let _chunk = Self {
            ..Default::default()
        };

        todo!("Not implemented");

        //chunk
    }

    fn serialize<W>(_chunk: &Self, _writer: &mut Writer<W>)
        where W: Write + WriteBytesExt + Seek,
    {
        todo!("Not implemented");
    }
}
