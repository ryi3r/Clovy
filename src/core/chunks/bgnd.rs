use crate::core::{reader::Reader, serializing::Serialize, writer::Writer, lists::GMPointerList, models::background::Background};
use byteorder::WriteBytesExt;
use std::{fmt::Write, io::{Read, Seek}};

#[derive(Default, Clone)]
pub struct ChunkBGND {
    pub backgrounds: GMPointerList<Background>,
}

impl Serialize for ChunkBGND {
    fn deserialize<R>(reader: &mut Reader<R>) -> Self
        where R: Read + Seek,
    {
        let mut chunk = Self {
            ..Default::default()
        };

        reader.version_info.align_backgrounds_to_8 = reader.version_info.is_version_at_least(2, 3, 0, 0);
        chunk.backgrounds.deserialize(reader, Some(Box::new(|reader: &mut Reader<R>, ptr: u64, _index: usize, _size: usize| {
            reader.version_info.align_backgrounds_to_8 &= ptr % 8 == 0;
        })), None);

        chunk
    }

    fn serialize<W>(_chunk: &Self, _writer: &mut Writer<W>)
        where W: Write + WriteBytesExt + Seek,
    {
        todo!("Not implemented")
    }
}
