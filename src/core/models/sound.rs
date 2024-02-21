use crate::core::{reader::Reader, serializing::Serialize, writer::Writer};
use bstr::BString;
use byteorder::WriteBytesExt;
use bitflags::bitflags;
use std::{fmt::Write, io::{Read, Seek}};

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
    fn deserialize<R>(reader: &mut Reader<R>) -> Self
        where R: Read + Seek,
    {
        let mut chunk = Self {
            ..Default::default()
        };

        chunk.name = reader.read_pointer_string().expect("Failed to read name");
        chunk.flags = AudioEntryFlags::from_bits_truncate(reader.read_u32().expect("Failed to read flags"));
        chunk.kind = reader.read_pointer_string_safe().expect("Failed to read kind");
        chunk.file = reader.read_pointer_string().expect("Failed to read file");
        chunk.effects = reader.read_u32().expect("Failed to read effects");
        chunk.volume = reader.read_f32().expect("Failed to read volume");
        chunk.pitch = reader.read_f32().expect("Failed to read pitch");
        if reader.version_info.format_id >= 14 {
            chunk.group_id = reader.read_i32().expect("Failed to read group id");
            chunk.audio_id = reader.read_i32().expect("Failed to read audio id");
        } else { // Legacy
            chunk.group_id = -1;
            chunk.audio_id = reader.read_i32().expect("Failed to read audio_id");
            chunk.preload = reader.read_wide_bool().expect("Failed to read preload");
        }

        chunk
    }

    fn serialize<W>(chunk: &Self, writer: &mut Writer<W>)
        where W: Write + WriteBytesExt + Seek,
    {
        writer.write_pointer_string(&chunk.name).expect("Failed to write name");
        writer.write_u32(chunk.flags.bits()).expect("Failed to write flags");
        writer.write_pointer_string(&chunk.kind).expect("Failed to write kind");
        writer.write_pointer_string(&chunk.file).expect("Failed to write file");
        writer.write_u32(chunk.effects).expect("Failed to write effects");
        writer.write_f32(chunk.volume).expect("Failed to write volume");
        writer.write_f32(chunk.pitch).expect("Failed to write pitch");
        if writer.version_info.format_id >= 14 {
            writer.write_i32(chunk.group_id).expect("Failed to write group id");
            writer.write_i32(chunk.audio_id).expect("Failed to write audio id");
        } else { // Legacy
            writer.write_i32(chunk.audio_id).expect("Failed to write audio_id");
            writer.write_wide_bool(chunk.preload).expect("Failed to write preload");
        }
    }
}
