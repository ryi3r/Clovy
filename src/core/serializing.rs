use super::{writer::Writer, reader::Reader};
use byteorder::WriteBytesExt;
use std::{fmt::Write, io::{Read, Result, Seek}};

pub trait Serialize {
    fn deserialize<R>(reader: &mut Reader<R>) -> Result<Self>
        where R: Read + Seek, Self: Sized;

    fn serialize<W>(chunk: &Self, writer: &mut Writer<W>) -> Result<()>
        where W: Write + WriteBytesExt + Seek;
    
}

pub trait FormatCheck {
    fn format_check<R>(reader: &mut Reader<R>) -> Result<()>
        where R: Read + Seek;
}