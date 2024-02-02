use bstr::BString;
use byteorder::{LittleEndian, WriteBytesExt};
use std::{collections::HashMap, io::{Result, Seek, SeekFrom, Write, Read}, path::PathBuf};
use super::{GMVersionInfo, GlobalData, reader::Reader, chunks::ChunkOutput};

pub struct Writer<T>
where
    T: Write + Seek,
{
    pub container: T,
    pub version_info: GMVersionInfo,
    pub chunks: HashMap<BString, ChunkOutput>,
    pub chunk_order: Vec<BString>,
    pub serialize_strings: HashMap<BString, Vec<u64>>,
    pub global_data: GlobalData,
    pub path: Option<PathBuf>,
}

impl<T> Writer<T>
where
    T: Write + Seek,
{
    pub fn new(container: T, path: Option<PathBuf>) -> Self {
        Self {
            container,
            version_info: GMVersionInfo::default(),
            chunks: HashMap::new(),
            chunk_order: Vec::new(),
            serialize_strings: HashMap::new(),
            global_data: GlobalData::default(),
            path,
        }
    }

    pub fn from_reader<R>(container: T, reader: &Reader<R>, path: Option<PathBuf>) -> Self
        where R: Read + Seek,
    {
        Self {
            container,
            version_info: reader.version_info.clone(),
            chunks: reader.chunks.clone(),
            chunk_order: reader.chunk_order.clone(),
            serialize_strings: HashMap::new(),
            global_data: reader.global_data.clone(),
            path,
        }
    }

    pub fn stream_position(&mut self) -> Result<u64> {
        self.container.stream_position()
    }

    pub fn seek(&mut self, pos: SeekFrom) -> Result<u64> {
        self.container.seek(pos)
    }

    pub fn seek_relative(&mut self, offset: i64) -> Result<u64> {
        self.container.seek(SeekFrom::Current(offset))
    }

    pub fn write(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.container.write(buf)
    }

    pub fn pad(&mut self, alignment: i64) -> Result<()> {
        let r = self.stream_position()? as i64 % alignment;
        if r != 0 {
            self.seek_relative(alignment - r)?;
        }
        Ok(())
    }

    pub fn pad_check_byte(&mut self, alignment: i64, byte: u8) -> Result<()> {
        while self.stream_position()? as i64 % alignment != 0 {
            self.write_u8(byte)?;
        }
        Ok(())
    }

    pub fn write_all(&mut self, buf: &[u8]) -> Result<()> {
        self.container.write_all(buf)
    }

    pub fn write_pointer<K>(&mut self, _value: K) -> Result<()> {
        todo!("Not Implemented");
    }

    pub fn write_bool(&mut self, value: bool) -> Result<()> {
        self.container.write_u8(value as u8)?;
        Ok(())
    }

    pub fn write_wide_bool(&mut self, value: bool) -> Result<()> {
        self.container.write_u32::<LittleEndian>(value as u32)?;
        Ok(())
    }

    pub fn write_bytes(&mut self, value: &[u8]) -> Result<()> {
        self.container.write_all(value)?;
        Ok(())
    }

    pub fn write_pointer_string(&mut self, string: &BString) -> Result<()> {
        if !self.serialize_strings.contains_key(string) {
            self.serialize_strings.insert(string.clone(), Vec::new());
        }
        self.serialize_strings.get_mut(string).unwrap().push(self.container.stream_position()?);
        self.container.write_u32::<LittleEndian>(0).unwrap();
        Ok(())
    }

    pub fn write_pointer_object<K>(&mut self, _value: K) -> Result<()> {
        todo!("Not Implemented");
    }

    pub fn write_u8(&mut self, value: u8) -> Result<()> {
        self.container.write_u8(value)
    }

    pub fn write_u16(&mut self, value: u16) -> Result<()> {
        self.container.write_u16::<LittleEndian>(value)
    }

    pub fn write_u32(&mut self, value: u32) -> Result<()> {
        self.container.write_u32::<LittleEndian>(value)
    }

    pub fn write_u64(&mut self, value: u64) -> Result<()> {
        self.container.write_u64::<LittleEndian>(value)
    }

    pub fn write_u128(&mut self, value: u128) -> Result<()> {
        self.container.write_u128::<LittleEndian>(value)
    }

    pub fn write_i8(&mut self, value: i8) -> Result<()> {
        self.container.write_i8(value)
    }

    pub fn write_i16(&mut self, value: i16) -> Result<()> {
        self.container.write_i16::<LittleEndian>(value)
    }

    pub fn write_i32(&mut self, value: i32) -> Result<()> {
        self.container.write_i32::<LittleEndian>(value)
    }

    pub fn write_i64(&mut self, value: i64) -> Result<()> {
        self.container.write_i64::<LittleEndian>(value)
    }

    pub fn write_i128(&mut self, value: i128) -> Result<()> {
        self.container.write_i128::<LittleEndian>(value)
    }

    pub fn write_f32(&mut self, value: f32) -> Result<()> {
        self.container.write_f32::<LittleEndian>(value)
    }

    pub fn write_f64(&mut self, value: f64) -> Result<()> {
        self.container.write_f64::<LittleEndian>(value)
    }
}
