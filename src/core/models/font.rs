use crate::core::{reader::Reader, serializing::Serialize, writer::Writer, lists::GMPointerList};
use bstr::BString;
use byteorder::WriteBytesExt;
use std::{fmt::Write, io::{Read, Result, Seek}};

#[derive(Default, Clone)]
pub struct Font {
    pub name: BString,
    pub display_name: BString,
    pub size: i32, // This from 2.3>= seems to be a float instead
    pub size_float: f32,
    pub bold: bool,
    pub italic: bool,
    pub range_start: u16,
    pub charset: i8,
    pub antialiasing: i8,
    pub range_end: i32,
    pub texture_item: i32, // TODO: Change it to a Texture Item once it's done
    pub scale_x: f32,
    pub scale_y: f32,
    pub ascender_offset: i32,
    pub ascender: i32,
    pub glyphs: GMPointerList<Glyph>,
}

impl Serialize for Font {
    fn deserialize<R>(reader: &mut Reader<R>) -> Result<Self>
        where R: Read + Seek,
    {
        let mut chunk = Self {
            ..Default::default()
        };

        chunk.name = reader.read_pointer_string()?;
        chunk.display_name = reader.read_pointer_string()?;
        chunk.size = reader.read_i32()?;
        if chunk.size < 0 {
            reader.seek_relative(-4)?;
            chunk.size_float = -reader.read_f32()?;
        }
        chunk.bold = reader.read_wide_bool()?;
        chunk.italic = reader.read_wide_bool()?;
        chunk.range_start = reader.read_u16()?;
        chunk.charset = reader.read_i8()?;
        chunk.antialiasing = reader.read_i8()?;
        chunk.range_end = reader.read_i32()?;
        chunk.texture_item = reader.read_i32()?;
        chunk.scale_x = reader.read_f32()?;
        chunk.scale_y = reader.read_f32()?;
        if reader.version_info.format_id >= 17 {
            chunk.ascender_offset = reader.read_i32()?;
        }
        if reader.version_info.is_version_at_least(2022, 2, 0, 0) {
            chunk.ascender = reader.read_i32()?;
        }
        chunk.glyphs.deserialize(reader, None, None)?;

        Ok(chunk)
    }

    fn serialize<W>(chunk: &Self, writer: &mut Writer<W>) -> Result<()>
        where W: Write + WriteBytesExt + Seek,
    {
        writer.write_pointer_string(&chunk.name)?;
        writer.write_pointer_string(&chunk.display_name)?;
        if chunk.size < 0 {
            writer.write_f32(-chunk.size_float)?;
        } else {
            writer.write_i32(chunk.size)?;
        }
        writer.write_wide_bool(chunk.bold)?;
        writer.write_wide_bool(chunk.italic)?;
        writer.write_u16(chunk.range_start)?;
        writer.write_i8(chunk.charset)?;
        writer.write_i8(chunk.antialiasing)?;
        writer.write_i32(chunk.range_end)?;
        writer.write_i32(chunk.texture_item)?;
        writer.write_f32(chunk.scale_x)?;
        writer.write_f32(chunk.scale_y)?;
        if writer.version_info.format_id >= 17 {
            writer.write_i32(chunk.ascender_offset)?;
        }
        if writer.version_info.is_version_at_least(2022, 2, 0, 0) {
            writer.write_i32(chunk.ascender)?;
        }
        chunk.glyphs.serialize(writer, None, None)?;

        Ok(())
    }
}

#[derive(Default, Clone)]
pub struct Glyph {
    pub character: u16,
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
    pub shift: i16,
    pub offset: i16,
    pub kerning: Vec<Kerning>,
}

impl Serialize for Glyph {
    fn deserialize<R>(reader: &mut Reader<R>) -> Result<Self>
        where R: Read + Seek,
    {
        let mut chunk = Self {
            ..Default::default()
        };

        chunk.character = reader.read_u16()?;
        chunk.x = reader.read_u16()?;
        chunk.y = reader.read_u16()?;
        chunk.width = reader.read_u16()?;
        chunk.height = reader.read_u16()?;
        chunk.shift = reader.read_i16()?;
        chunk.offset = reader.read_i16()?;
        for _ in 0..reader.read_u16()? {
            chunk.kerning.push(Kerning::deserialize(reader)?);
        }

        Ok(chunk)
    }

    fn serialize<W>(chunk: &Self, writer: &mut Writer<W>) -> Result<()>
        where W: Write + WriteBytesExt + Seek,
    {
        writer.write_u16(chunk.character)?;
        writer.write_u16(chunk.x)?;
        writer.write_u16(chunk.y)?;
        writer.write_u16(chunk.width)?;
        writer.write_u16(chunk.height)?;
        writer.write_i16(chunk.shift)?;
        writer.write_i16(chunk.offset)?;
        writer.write_u16(chunk.kerning.len() as u16)?;
        for kerning in chunk.kerning.iter() {
            Kerning::serialize(kerning, writer)?;
        }

        Ok(())
    }
}

#[derive(Default, Clone)]
pub struct Kerning {
    pub other: i16,
    pub amount: i16,
}

impl Serialize for Kerning {
    fn deserialize<R>(reader: &mut Reader<R>) -> Result<Self>
        where R: Read + Seek,
    {
        let mut chunk = Self {
            ..Default::default()
        };

        chunk.other = reader.read_i16()?;
        chunk.amount = reader.read_i16()?;

        Ok(chunk)
    }

    fn serialize<W>(chunk: &Self, writer: &mut Writer<W>) -> Result<()>
        where W: Write + WriteBytesExt + Seek,
    {
        writer.write_i16(chunk.other)?;
        writer.write_i16(chunk.amount)?;

        Ok(())
    }
}
