use crate::core::{reader::Reader, serializing::Serialize, writer::Writer, lists::GMPointerList};
use bitflags::bitflags;
use bstr::BString;
use byteorder::WriteBytesExt;
use tracing::warn;
use std::{fmt::Write, io::{Read, Result, Seek}};

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct SepMaskType: i32 {
        const AxisAlignedRect = 0;
        const Precise = 1;
        const RotatedRect = 2;
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct SpriteType: i32 {
        const Normal = 0;
        const Swf = 1;
        const Spine = 2;
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct AnimSpeedType: i32 {
        const FramesPerSecond = 0;
        const FramesPerGameFrame = 1;
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct TileMode: i32 {
        const Stretch = 0;
        const Repeat = 1;
        const Mirror = 2;
        const BlankRepeat = 3;
        const Hide = 4;
    }
}

impl Default for SepMaskType {
    fn default() -> Self {
        Self::AxisAlignedRect
    }
}

impl Default for SpriteType {
    fn default() -> Self {
        Self::Normal
    }
}

impl Default for AnimSpeedType {
    fn default() -> Self {
        Self::FramesPerSecond
    }
}

impl Default for TileMode {
    fn default() -> Self {
        Self::Stretch
    }
}

#[derive(Default, Clone)]
pub struct Sprite {
    pub name: BString,
    pub width: i32,
    pub height: i32,
    pub margin_left: i32,
    pub margin_right: i32,
    pub margin_bottom: i32,
    pub margin_top: i32,
    pub transparent: bool,
    pub smooth: bool,
    pub preload: bool,
    pub bbox_mode: u32,
    pub sep_masks: SepMaskType,
    pub origin_x: i32,
    pub origin_y: i32,
    pub special_or_gms2: bool,
    pub sprite_type: SpriteType,
    pub buffer: Vec<u8>,
    pub gms2_playback_speed: f32,
    pub gms2_playback_speed_type: AnimSpeedType,
    pub gms2_3_sequence: u32, //SequenceReference,
    pub gms2_3_2_nine_slice: NineSlice,
    pub texture_items: GMPointerList<BString>, // TODO: Change this to Texture Item
    pub collision_masks: Vec<Vec<u8>>,
}

impl Serialize for Sprite {
    fn deserialize<R>(reader: &mut Reader<R>) -> Result<Self>
        where R: Read + Seek,
    {
        let mut chunk = Self {
            ..Default::default()
        };

        chunk.name = reader.read_pointer_string()?;
        chunk.width = reader.read_i32()?;
        chunk.height = reader.read_i32()?;
        chunk.margin_left = reader.read_i32()?;
        chunk.margin_right = reader.read_i32()?;
        chunk.margin_bottom = reader.read_i32()?;
        chunk.margin_top = reader.read_i32()?;
        chunk.transparent = reader.read_bool()?;
        chunk.smooth = reader.read_bool()?;
        chunk.preload = reader.read_bool()?;
        chunk.bbox_mode = reader.read_u32()?;
        chunk.sep_masks = SepMaskType::from_bits_truncate(reader.read_i32()?);
        chunk.origin_x = reader.read_i32()?;
        chunk.origin_y = reader.read_i32()?;
        if reader.read_i32()? == -1 {
            chunk.special_or_gms2 = true;
            let version = reader.read_i32()?;
            chunk.sprite_type = SpriteType::from_bits_retain(reader.read_i32()?);
            if reader.version_info.is_version_at_least(2, 0, 0, 0) {
                chunk.gms2_playback_speed = reader.read_f32()?;
                chunk.gms2_playback_speed_type = AnimSpeedType::from_bits_retain(reader.read_i32()?);
                if version >= 2 {
                    //chunk.gms2_3_sequence = SequenceReference::deserialize(reader);
                    chunk.gms2_3_sequence = reader.read_u32()?;
                    if version >= 3 {
                        reader.version_info.set_version(2, 3, 2, 0);
                        chunk.gms2_3_2_nine_slice = reader.read_pointer_object::<NineSlice>()?;
                    }
                }
            }

            // TODO: Finish this up
            match chunk.sprite_type {
                SpriteType::Normal => {

                }
                SpriteType::Swf => {

                }
                SpriteType::Spine => {

                }
                _ => {
                    panic!("Unexpected sprite type");
                }
            }
        } else {
            reader.seek_relative(-4)?;
        }

        Ok(chunk)
    }

    fn serialize<W>(_chunk: &Self, _writer: &mut Writer<W>) -> Result<()>
        where W: Write + WriteBytesExt + Seek,
    {
        todo!("Not implemented");
    }
}

#[derive(Default, Clone)]
pub struct SequenceReference {
}

impl Serialize for SequenceReference {
    fn deserialize<R>(reader: &mut Reader<R>) -> Result<Self>
        where R: Read + Seek,
    {
        let _chunk = Self {
            //..Default::default()
        };

        if reader.read_i32()? != 1 {
            warn!("Unexpected version for sequence reference in Sprite.");
        }

        todo!("Not implemented."); // This is quite literally a Sequence, not a pointer reference

        //chunk
    }

    fn serialize<W>(_chunk: &Self, _writer: &mut Writer<W>) -> Result<()>
        where W: Write + WriteBytesExt + Seek,
    {
        todo!("Not implemented.");
    }
}

#[derive(Default, Clone)]
pub struct NineSlice {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
    pub enabled: bool,
    pub tile_modes: Vec<TileMode>,
}

impl Serialize for NineSlice {
    fn deserialize<R>(reader: &mut Reader<R>) -> Result<Self>
        where R: Read + Seek,
    {
        let mut chunk = Self {
            ..Default::default()
        };

        chunk.left = reader.read_i32()?;
        chunk.top = reader.read_i32()?;
        chunk.right = reader.read_i32()?;
        chunk.bottom = reader.read_i32()?;
        chunk.enabled = reader.read_wide_bool()?;
        for _ in 0..5 {
            chunk.tile_modes.push(TileMode::from_bits_retain(reader.read_i32()?));
        }

        Ok(chunk)
    }

    fn serialize<W>(chunk: &Self, writer: &mut Writer<W>) -> Result<()>
        where W: Write + WriteBytesExt + Seek,
    {
        writer.write_i32(chunk.left)?;
        writer.write_i32(chunk.top)?;
        writer.write_i32(chunk.right)?;
        writer.write_i32(chunk.bottom)?;
        writer.write_wide_bool(chunk.enabled)?;
        for tile_mode in chunk.tile_modes.iter() {
            writer.write_i32(tile_mode.bits())?;
        }

        Ok(())
    }
}
