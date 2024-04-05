use crate::core::{reader::Reader, serializing::{Serialize, FormatCheck}, writer::Writer, lists::GMPointerList, models::extension::Extension};
use byteorder::WriteBytesExt;
use std::{fmt::Write, io::{Read, Result, Seek, SeekFrom}};

#[derive(Default, Clone)]
pub struct ChunkEXTN {
    pub extensions: GMPointerList<Extension>,
}

impl FormatCheck for ChunkEXTN {
    fn format_check<R>(reader: &mut Reader<R>) -> Result<()>
        where R: Read + Seek,
    {
        if reader.version_info.is_version_at_least(2, 3, 0, 0) && !reader.version_info.is_version_at_least(2022, 6, 0, 0) {
            let mut definitively_2022_6 = true;
            let return_to = reader.stream_position()?;

            let extn_count = reader.read_u32()?;
            if extn_count > 0 {
                let first_extn_ptr = reader.read_u32()?;
                let first_extn_end_ptr = {
                    if extn_count >= 2 {
                        reader.read_u32()? as u64
                    } else {
                        reader.current_chunk.end_offset
                    }
                };

                reader.seek(SeekFrom::Start((first_extn_ptr + 12) as _))?;
                let new_pointer_1 = reader.read_u32()?;
                let new_pointer_2 = reader.read_u32()?;

                if (new_pointer_1 != reader.stream_position()? as _) || (new_pointer_2 <= reader.stream_position()? as _ || new_pointer_2 as u64 >= reader.current_chunk.end_offset) {
                    definitively_2022_6 = false;
                } else {
                    reader.seek(SeekFrom::Start(new_pointer_2 as _))?;
                    let option_count = reader.read_u32()?;
                    if option_count > 0 {
                        let new_offset_check = reader.stream_position()? + (4 * (option_count as u64 - 1));
                        if new_offset_check >= reader.current_chunk.end_offset {
                            definitively_2022_6 = false;
                        } else {
                            reader.seek_relative(4 * (option_count as i64 - 1))?;
                            let new_offset_check = reader.read_i32()? + 12;
                            if new_offset_check < 0 || new_offset_check >= reader.current_chunk.end_offset as _ {
                                definitively_2022_6 = false;
                            } else {
                                reader.seek(SeekFrom::Start(new_offset_check as _))?;
                            }
                        }
                    }
                    if definitively_2022_6 {
                        if extn_count == 1 {
                            reader.seek_relative(16)?; // Skip GUID data
                            reader.pad(16)?;
                        }
                        if reader.stream_position()? != first_extn_end_ptr {
                            definitively_2022_6 = false;
                        }
                    }
                }
            } else {
                definitively_2022_6 = false;
            }
            reader.seek(SeekFrom::Start(return_to))?;
            if definitively_2022_6 {
                reader.version_info.set_version(2022, 6, 0, 0);
            }
        }

        Ok(())
    }
}

impl Serialize for ChunkEXTN {
    fn deserialize<R>(reader: &mut Reader<R>) -> Result<Self>
        where R: Read + Seek,
    {
        let mut chunk = Self {
            ..Default::default()
        };

        chunk.extensions.deserialize(reader, None, None)?;
        if reader.version_info.is_version_at_least(1, 0, 0, 9999) {
            for extension in chunk.extensions.values.iter_mut() {
                extension.guid = Some(reader.read_bytes::<16>()?);
            }
        }

        Ok(chunk)
    }

    fn serialize<W>(chunk: &Self, writer: &mut Writer<W>) -> Result<()>
        where W: Write + WriteBytesExt + Seek,
    {
        chunk.extensions.serialize(writer, None, None)?;
        for extension in chunk.extensions.values.iter() {
            if let Some(guid) = &extension.guid {
                writer.write_bytes(guid)?;
            }
        }

        Ok(())
    }
}