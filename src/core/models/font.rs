use crate::core::{reader::Reader, serializing::Serialize, writer::Writer, lists::GMPointerList};
use bstr::BString;
use byteorder::WriteBytesExt;
use std::{fmt::Write, io::{Read, Seek}};

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
    fn deserialize<R>(reader: &mut Reader<R>) -> Self
        where R: Read + Seek,
    {
        let mut chunk = Self {
            ..Default::default()
        };

        chunk.name = reader.read_pointer_string().expect("Failed to read name");
        chunk.display_name = reader.read_pointer_string().expect("Failed to read display_name");
        chunk.size = reader.read_i32().expect("Failed to read size");
        if chunk.size < 0 {
            reader.seek_relative(-4).expect("Failed to seek back");
            chunk.size_float = -reader.read_f32().expect("Failed to read size_float");
        }
        chunk.bold = reader.read_wide_bool().expect("Failed to read bold");
        chunk.italic = reader.read_wide_bool().expect("Failed to read italic");
        chunk.range_start = reader.read_u16().expect("Failed to read range_start");
        chunk.charset = reader.read_i8().expect("Failed to read charset");
        chunk.antialiasing = reader.read_i8().expect("Failed to read antialiasing");
        chunk.range_end = reader.read_i32().expect("Failed to read range_end");
        chunk.texture_item = reader.read_i32().expect("Failed to read texture_item");
        chunk.scale_x = reader.read_f32().expect("Failed to read scale_x");
        chunk.scale_y = reader.read_f32().expect("Failed to read scale_y");
        if reader.version_info.format_id >= 17 {
            chunk.ascender_offset = reader.read_i32().expect("Failed to read ascender_offset");
        }
        if reader.version_info.is_version_at_least(2022, 2, 0, 0) {
            chunk.ascender = reader.read_i32().expect("Failed to read ascender");
        }
        chunk.glyphs.deserialize(reader, None, None);

        chunk
    }

    fn serialize<W>(chunk: &Self, writer: &mut Writer<W>)
        where W: Write + WriteBytesExt + Seek,
    {
        writer.write_pointer_string(&chunk.name).expect("Failed to write name");
        writer.write_pointer_string(&chunk.display_name).expect("Failed to write display_name");
        if chunk.size < 0 {
            writer.write_f32(-chunk.size_float).expect("Failed to write size_float");
        } else {
            writer.write_i32(chunk.size).expect("Failed to write size");
        }
        writer.write_wide_bool(chunk.bold).expect("Failed to write bold");
        writer.write_wide_bool(chunk.italic).expect("Failed to write italic");
        writer.write_u16(chunk.range_start).expect("Failed to write range_start");
        writer.write_i8(chunk.charset).expect("Failed to write charset");
        writer.write_i8(chunk.antialiasing).expect("Failed to write antialiasing");
        writer.write_i32(chunk.range_end).expect("Failed to write range_end");
        writer.write_i32(chunk.texture_item).expect("Failed to write texture_item");
        writer.write_f32(chunk.scale_x).expect("Failed to write scale_x");
        writer.write_f32(chunk.scale_y).expect("Failed to write scale_y");
        if writer.version_info.format_id >= 17 {
            writer.write_i32(chunk.ascender_offset).expect("Failed to write ascender_offset");
        }
        if writer.version_info.is_version_at_least(2022, 2, 0, 0) {
            writer.write_i32(chunk.ascender).expect("Failed to write ascender");
        }
        chunk.glyphs.serialize(writer, None, None);
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
    fn deserialize<R>(reader: &mut Reader<R>) -> Self
        where R: Read + Seek,
    {
        let mut chunk = Self {
            ..Default::default()
        };

        chunk.character = reader.read_u16().expect("Failed to read character");
        chunk.x = reader.read_u16().expect("Failed to read x");
        chunk.y = reader.read_u16().expect("Failed to read y");
        chunk.width = reader.read_u16().expect("Failed to read width");
        chunk.height = reader.read_u16().expect("Failed to read height");
        chunk.shift = reader.read_i16().expect("Failed to read shift");
        chunk.offset = reader.read_i16().expect("Failed to read offset");
        for _ in 0..reader.read_u16().expect("Failed to read kerning size") {
            chunk.kerning.push(Kerning::deserialize(reader));
        }

        chunk
    }

    fn serialize<W>(chunk: &Self, writer: &mut Writer<W>)
        where W: Write + WriteBytesExt + Seek,
    {
        writer.write_u16(chunk.character).expect("Failed to write character");
        writer.write_u16(chunk.x).expect("Failed to write x");
        writer.write_u16(chunk.y).expect("Failed to write y");
        writer.write_u16(chunk.width).expect("Failed to write width");
        writer.write_u16(chunk.height).expect("Failed to write height");
        writer.write_i16(chunk.shift).expect("Failed to write shift");
        writer.write_i16(chunk.offset).expect("Failed to write offset");
        writer.write_u16(chunk.kerning.len() as u16).expect("Failed to write kerning size");
        for kerning in chunk.kerning.iter() {
            Kerning::serialize(kerning, writer);
        }
    }
}

#[derive(Default, Clone)]
pub struct Kerning {
    pub other: i16,
    pub amount: i16,
}

impl Serialize for Kerning {
    fn deserialize<R>(reader: &mut Reader<R>) -> Self
        where R: Read + Seek,
    {
        let mut chunk = Self {
            ..Default::default()
        };

        chunk.other = reader.read_i16().expect("Failed to read other");
        chunk.amount = reader.read_i16().expect("Failed to read amount");

        chunk
    }

    fn serialize<W>(chunk: &Self, writer: &mut Writer<W>)
        where W: Write + WriteBytesExt + Seek,
    {
        writer.write_i16(chunk.other).expect("Failed to write other");
        writer.write_i16(chunk.amount).expect("Failed to write amount");
    }
}
