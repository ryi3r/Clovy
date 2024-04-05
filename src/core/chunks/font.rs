use crate::core::{reader::Reader, serializing::{Serialize, FormatCheck}, writer::Writer, lists::GMPointerList, models::font::Font};
use byteorder::WriteBytesExt;
use std::{fmt::Write, io::{Read, Result, Seek, SeekFrom}};

#[derive(Default, Clone)]
pub struct ChunkFONT {
    pub fonts: GMPointerList<Font>,
    pub padding: Option<[u8; 512]>,
}

impl FormatCheck for ChunkFONT {
    fn format_check<R>(reader: &mut Reader<R>) -> Result<()>
        where R: Read + Seek,
    {
        if reader.version_info.is_version_at_least(2, 3, 0, 0) && !reader.version_info.is_version_at_least(2022, 2, 0, 0) {
            let return_to = reader.stream_position()?;
            let font_count = reader.read_u32()?;
            if font_count > 0 {
                let lower_bound = reader.stream_position()?;
                let upper_bound = reader.current_chunk.end_offset - 512;
                let first_font_ptr = reader.read_i32()?;
                let end_ptr = if font_count >= 2 {
                    reader.read_u32()?
                } else {
                    upper_bound as _
                };
                reader.seek(SeekFrom::Start(first_font_ptr as u64 + (11 * 4)))?;
                let glyph_count = reader.read_u32()?;
                let mut invalid_format = false;
                if glyph_count > 0 {
                    let glyph_ptr_offset = reader.stream_position()?;
                    if glyph_count >= 2 {
                        let first_glyph = reader.read_u32()? + (7 * 2);
                        let second_glyph = reader.read_u32()?;
                        if (first_glyph as u64) < lower_bound || (first_glyph as u64) > upper_bound || (second_glyph as u64) < lower_bound || (second_glyph as u64) > upper_bound {
                            invalid_format = true;
                        }
                        if !invalid_format {
                            reader.seek(SeekFrom::Start(first_glyph as _))?;
                            let kerning_length = reader.read_u16()? * 4;
                            reader.seek_relative(kerning_length as _)?;
                            if reader.stream_position()? != second_glyph as _ {
                                invalid_format = true;
                            }
                        }
                    }
                    if !invalid_format {
                        reader.seek(SeekFrom::Start(glyph_ptr_offset + ((glyph_count as u64 - 1) * 4)))?;
                        let last_glyph = reader.read_u32()?;
                        if (last_glyph as u64) < lower_bound || (last_glyph as u64) > upper_bound {
                            invalid_format = true;
                        }
                        if !invalid_format {
                            reader.seek(SeekFrom::Start(last_glyph as _))?;
                            let kerning_length = reader.read_u16()? * 4;
                            reader.seek_relative(kerning_length as _)?;
                            if font_count == 1 && reader.version_info.align_chunks_to_16 {
                                reader.pad(16)?;
                            }
                        }
                    }
                }
                if invalid_format || reader.stream_position()? != end_ptr as _ {
                    reader.version_info.set_version(2022, 2, 0, 0);
                }
            }
            reader.seek(SeekFrom::Start(return_to))?;
        }

        Ok(())
    }
}

impl Serialize for ChunkFONT {
    fn deserialize<R>(reader: &mut Reader<R>) -> Result<Self>
        where R: Read + Seek,
    {
        let mut chunk = Self {
            ..Default::default()
        };

        chunk.fonts.deserialize(reader, None, None)?;
        chunk.padding = Some(reader.read_bytes::<512>()?);

        Ok(chunk)
    }

    fn serialize<W>(chunk: &Self, writer: &mut Writer<W>) -> Result<()>
        where W: Write + WriteBytesExt + Seek,
    {
        chunk.fonts.serialize(writer, None, None)?;
        if let Some(padding) = chunk.padding {
            writer.write_bytes(&padding)?;
        } else {
            for i in 0u16..0x80 {
                writer.write_u16(i)?;
            }
            for _ in 0..0x80 {
                writer.write_u16(0x3f)?;
            }
        }

        Ok(())
    }
}
