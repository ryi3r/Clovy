use crate::core::{reader::Reader, serializing::{Serialize, FormatCheck}, writer::Writer, lists::GMPointerList, models::extension::Extension};
use byteorder::WriteBytesExt;
use std::{fmt::Write, io::{Read, Seek, SeekFrom}};

#[derive(Default, Clone)]
pub struct ChunkEXTN {
    pub extensions: GMPointerList<Extension>,
}

impl FormatCheck for ChunkEXTN {
    fn do_format_check<R>(reader: &mut Reader<R>)
        where R: Read + Seek,
    {
        if reader.version_info.is_version_at_least(2, 3, 0, 0) && !reader.version_info.is_version_at_least(2022, 6, 0, 0) {
            let mut definitively_2022_6 = true;
            let return_to = reader.stream_position().expect("Failed to get stream position");

            let extn_count = reader.read_u32().expect("Failed to read extension count");
            if extn_count > 0 {
                let first_extn_ptr = reader.read_u32().expect("Failed to read first extension pointer");
                let first_extn_end_ptr = {
                    if extn_count >= 2 {
                        reader.read_u32().expect("Failed to read second extension pointer") as u64
                    } else {
                        reader.current_chunk.end_offset
                    }
                };

                reader.seek(SeekFrom::Start((first_extn_ptr + 12) as _)).expect("Failed to seek to the first extension pointer + 12");
                let new_pointer_1 = reader.read_u32().expect("Failed to read new pointer 1");
                let new_pointer_2 = reader.read_u32().expect("Failed to read new pointer 2");

                if (new_pointer_1 != reader.stream_position().expect("Failed to get stream position") as _) || (new_pointer_2 <= reader.stream_position().expect("Failed to get stream position") as _ || new_pointer_2 as u64 >= reader.current_chunk.end_offset) {
                    definitively_2022_6 = false;
                } else {
                    reader.seek(SeekFrom::Start(new_pointer_2 as _)).expect("Failed to seek to new pointer 2");
                    let option_count = reader.read_u32().expect("Failed to read option_count");
                    if option_count > 0 {
                        let new_offset_check = reader.stream_position().expect("Failed to get stream position") + (4 * (option_count as u64 - 1));
                        if new_offset_check >= reader.current_chunk.end_offset {
                            definitively_2022_6 = false;
                        } else {
                            reader.seek_relative(4 * (option_count as i64 - 1)).expect("Failed to seek to new offset check");
                            let new_offset_check = reader.read_i32().expect("Failed to read new offset check") + 12;
                            if new_offset_check < 0 || new_offset_check >= reader.current_chunk.end_offset as _ {
                                definitively_2022_6 = false;
                            } else {
                                reader.seek(SeekFrom::Start(new_offset_check as _)).expect("Failed to seek to new offset check");
                            }
                        }
                    }
                    if definitively_2022_6 {
                        if extn_count == 1 {
                            reader.seek_relative(16).expect("Failed to skip the GUID data (16 bytes)"); // Skip GUID data
                            reader.pad(16).expect("Failed to align to the chunk end");
                        }
                        if reader.stream_position().expect("Failed to get the stream position") != first_extn_end_ptr {
                            definitively_2022_6 = false;
                        }
                    }
                }
            } else {
                definitively_2022_6 = false;
            }
            reader.seek(SeekFrom::Start(return_to)).expect("Failed to seek to the return position");
            if definitively_2022_6 {
                reader.version_info.set_version(2022, 6, 0, 0);
            }
        }
    }
}

impl Serialize for ChunkEXTN {
    fn deserialize<R>(reader: &mut Reader<R>) -> Self
        where R: Read + Seek,
    {
        let mut chunk = Self {
            ..Default::default()
        };

        chunk.extensions.deserialize(reader, None, None);
        if reader.version_info.is_version_at_least(1, 0, 0, 9999) {
            for extension in chunk.extensions.values.iter_mut() {
                extension.guid = Some(reader.read_bytes::<16>().expect("Failed to read guid"));
            }
        }

        chunk
    }

    fn serialize<W>(chunk: &Self, writer: &mut Writer<W>)
        where W: Write + WriteBytesExt + Seek,
    {
        chunk.extensions.serialize(writer, None, None);
        for extension in chunk.extensions.values.iter() {
            if let Some(guid) = &extension.guid {
                writer.write_bytes(guid).expect("Failed to write guid");
            }
        }
    }
}