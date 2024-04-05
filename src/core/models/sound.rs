use crate::core::{reader::Reader, serializing::Serialize, writer::Writer};
use bstr::BString;
use byteorder::WriteBytesExt;
use bitflags::bitflags;
use std::{fmt::Write, io::{Read, Result, Seek}};

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct AudioEntryFlags: u32 {
        const IsEmbedded = 0x1;
        const IsCompressed = 0x2;
        const Regular = 0x64;
    }
}

#[derive(Default, Clone)]
pub struct Sound {
    pub name: BString,
    pub flags: AudioEntryFlags,
    pub kind: BString,
    pub file: BString,
    pub effects: u32,
    pub volume: f32,
    pub pitch: f32,
    pub audio_id: i32,
    pub group_id: i32,
    pub preload: bool,
}

impl Default for AudioEntryFlags {
    fn default() -> Self {
        Self::Regular
    }
}

impl Serialize for Sound {
    fn deserialize<R>(reader: &mut Reader<R>) -> Result<Self>
        where R: Read + Seek,
    {
        let mut chunk = Self {
            ..Default::default()
        };

        chunk.name = reader.read_pointer_string()?;
        chunk.flags = AudioEntryFlags::from_bits_truncate(reader.read_u32()?);
        chunk.kind = reader.read_pointer_string_safe()?;
        chunk.file = reader.read_pointer_string()?;
        chunk.effects = reader.read_u32()?;
        chunk.volume = reader.read_f32()?;
        chunk.pitch = reader.read_f32()?;
        if reader.version_info.format_id >= 14 {
            chunk.group_id = reader.read_i32()?;
            chunk.audio_id = reader.read_i32()?;
        } else { // Legacy
            chunk.group_id = -1;
            chunk.audio_id = reader.read_i32()?;
            chunk.preload = reader.read_wide_bool()?;
        }

        Ok(chunk)
    }

    fn serialize<W>(chunk: &Self, writer: &mut Writer<W>) -> Result<()>
        where W: Write + WriteBytesExt + Seek,
    {
        writer.write_pointer_string(&chunk.name)?;
        writer.write_u32(chunk.flags.bits())?;
        writer.write_pointer_string(&chunk.kind)?;
        writer.write_pointer_string(&chunk.file)?;
        writer.write_u32(chunk.effects)?;
        writer.write_f32(chunk.volume)?;
        writer.write_f32(chunk.pitch)?;
        if writer.version_info.format_id >= 14 {
            writer.write_i32(chunk.group_id)?;
            writer.write_i32(chunk.audio_id)?;
        } else { // Legacy
            writer.write_i32(chunk.audio_id)?;
            writer.write_wide_bool(chunk.preload)?;
        }

        Ok(())
    }
}
