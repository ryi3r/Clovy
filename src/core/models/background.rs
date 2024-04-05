use crate::core::{reader::Reader, serializing::Serialize, writer::Writer};
use bstr::BString;
use byteorder::WriteBytesExt;
use tracing::warn;
use std::{fmt::Write, io::{Read, Result, Seek}};

#[derive(Default, Clone)]
pub struct Background {
    pub name: BString,
    pub transparent: bool,
    pub smooth: bool,
    pub preload: bool,
    pub texture_item: u32, // TODO: Set this to Texture Item once it's finished
    pub tile_unknown1: u32, // Seems to always be 2, currently unknown (maybe it's tile version?)
    pub tile_width: u32, // GMS2 only
    pub tile_height: u32, // GMS2 only
    pub tile_output_border_x: u32, // GMS2 only
    pub tile_output_border_y: u32, // GMS2 only
    pub tile_columns: u32, // GMS2 only
    pub tile_unknown2: u32, // Seems to always be 0, currently unknown
    pub tile_frame_length: i64, // Time in microseconds, GMS2 only
    pub tiles: Vec<Vec<u32>>, // Entries per tile per frame, GMS2 only
}

impl Serialize for Background {
    fn deserialize<R>(reader: &mut Reader<R>) -> Result<Self>
        where R: Read + Seek,
    {
        let mut chunk = Self {
            ..Default::default()
        };

        chunk.name = reader.read_pointer_string()?;
        chunk.transparent = reader.read_wide_bool()?;
        chunk.smooth = reader.read_wide_bool()?;
        chunk.preload = reader.read_wide_bool()?;
        chunk.texture_item = reader.read_u32()?;

        if reader.version_info.major >= 2 {
            chunk.tile_unknown1 = reader.read_u32()?;
            if chunk.tile_unknown1 != 2 {
                warn!("Expected 2 in BGND");
            }
            chunk.tile_width = reader.read_u32()?;
            chunk.tile_height = reader.read_u32()?;
            chunk.tile_output_border_x = reader.read_u32()?;
            chunk.tile_output_border_y = reader.read_u32()?;
            chunk.tile_columns = reader.read_u32()?;
            let tile_frame_count = reader.read_u32()?;
            let tile_count = reader.read_u32()?;
            chunk.tile_unknown2 = reader.read_u32()?;
            if chunk.tile_unknown2 != 0 {
                warn!("Expected 0 in BGND");
            }
            chunk.tile_frame_length = reader.read_i64()?;
            for _ in 0..tile_count {
                let mut tile_frames = Vec::new();
                for _ in 0..tile_frame_count {
                    tile_frames.push(reader.read_u32()?);
                }
                chunk.tiles.push(tile_frames);
            }
        }

        Ok(chunk)
    }

    fn serialize<W>(chunk: &Self, writer: &mut Writer<W>) -> Result<()>
        where W: Write + WriteBytesExt + Seek,
    {
        writer.write_pointer_string(&chunk.name)?;
        writer.write_wide_bool(chunk.transparent)?;
        writer.write_wide_bool(chunk.smooth)?;
        writer.write_wide_bool(chunk.preload)?;
        writer.write_u32(chunk.texture_item)?;

        if writer.version_info.major >= 2 {
            writer.write_u32(chunk.tile_unknown1)?;
            writer.write_u32(chunk.tile_width)?;
            writer.write_u32(chunk.tile_height)?;
            writer.write_u32(chunk.tile_output_border_x)?;
            writer.write_u32(chunk.tile_output_border_y)?;
            writer.write_u32(chunk.tile_columns)?;
            writer.write_u32(chunk.tiles[0].len() as u32)?;
            writer.write_u32(chunk.tiles.len() as u32)?;
            writer.write_u32(chunk.tile_unknown2)?;
            writer.write_i64(chunk.tile_frame_length)?;
            for (index, tile_frames) in chunk.tiles.iter().enumerate() {
                if index != 0 && chunk.tiles[index].len() != chunk.tiles[index - 1].len() {
                    warn!("Amount of frames is different across tiles");
                }
                for item in tile_frames.iter() {
                    writer.write_u32(*item)?;
                }
            }
        }

        Ok(())
    }
}
