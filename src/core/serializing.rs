use super::{writer::Writer, reader::Reader};
use byteorder::WriteBytesExt;
use std::{fmt::Write, io::{Seek, Read}};

pub trait Serialize {
    fn deserialize<R>(reader: &mut Reader<R>) -> Self
        where R: Read + Seek;

    fn serialize<W>(chunk: &Self, writer: &mut Writer<W>)
        where W: Write + WriteBytesExt + Seek;
    
}

pub trait FormatCheck {
    fn do_format_check<R>(reader: &mut Reader<R>)
        where R: Read + Seek;
}