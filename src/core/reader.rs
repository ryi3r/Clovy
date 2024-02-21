use crate::core::chunks::{dummy::DummyChunk, gen8::ChunkGEN8, ChunkOutput, optn::ChunkOPTN, lang::ChunkLANG, extn::ChunkEXTN, sond::ChunkSOND, agrp::ChunkAGRP, sprt::ChunkSPRT, bgnd::ChunkBGND, path::ChunkPATH, scpt::ChunkSCPT, glob::ChunkGLOB, shdr::ChunkSHDR, font::ChunkFONT, tmln::ChunkTMLN, objt::ChunkOBJT, feds::ChunkFEDS, acrv::ChunkACRV, seqn::ChunkSEQN};
use bstr::{BString, ByteSlice};
use byteorder::{LittleEndian, ReadBytesExt};
use std::{collections::HashMap, io::{Error, ErrorKind, Read, Result, Seek, SeekFrom}, path::PathBuf};
use tracing::{info, error};
use super::{GMVersionInfo, Chunk, GlobalData, serializing::{Serialize, FormatCheck}};

#[derive(Clone)]
pub struct Reader<T>
    where T: Read + Seek,
{
    pub container: T,
    pub version_info: GMVersionInfo,
    pub chunks: HashMap<BString, ChunkOutput>,
    pub chunk_order: Vec<BString>,
    pub chunk_data: HashMap<BString, Chunk>,
    pub current_chunk: Chunk,
    pub global_data: GlobalData,
    pub path: Option<PathBuf>,
}

impl<T> Reader<T>
    where T: Read + Seek + ReadBytesExt,
{
    pub fn new(container: T, path: Option<PathBuf>) -> Self {
        Self {
            container,
            version_info: GMVersionInfo::default(),
            chunks: HashMap::new(),
            chunk_order: Vec::new(),
            chunk_data: HashMap::new(),
            current_chunk: Chunk::default(),
            global_data: GlobalData::default(),
            path,
        }
    }

    pub fn deserialize_chunks(&mut self) -> Result<()> {
        self.chunk_order.clear();
        self.chunk_data.clear();
        let start_pos = self.container.stream_position()?;
        self.container.seek(SeekFrom::Start(4))?; // Skip "FORM" name
        let size = self.container.read_u32::<LittleEndian>()? as u64 + 4;
        while self.container.stream_position()? < size {
            let chunk_name = BString::new(
                self.container
                    .read_u32::<LittleEndian>()?
                    .to_le_bytes()
                    .into(),
            );
            let chunk_size = self.container.read_i32::<LittleEndian>()?;
            match chunk_name.to_str().expect("Chunk name is not valid UTF-8") {
                "EXTN" => {
                    ChunkEXTN::do_format_check(self);
                }
                "FONT" => {
                    ChunkFONT::do_format_check(self);
                }
                _ => {}
            }
            self.chunk_order.push(chunk_name.clone());
            self.chunk_data.insert(chunk_name.clone(), Chunk {
                name: chunk_name.clone(),
                length: chunk_size as _,
                start_offset: self.container.stream_position()?,
                end_offset: self.container.stream_position()? + chunk_size as u64,
            });
            self.chunks.entry(chunk_name).or_insert(ChunkOutput::DummyChunk(DummyChunk {
                ..Default::default()
            }));
            self.container.seek(SeekFrom::Current(chunk_size as _))?;
        }
        self.container.seek(SeekFrom::Start(start_pos))?;
        Ok(())
    }

    pub fn deserialize(&mut self) -> Result<()> {
        macro_rules! deserialize_chunk {
            ($name: expr, $ctype: ty) => {
                let value = <$ctype>::deserialize(self);
                self.chunks.insert($name, value.into());
            }
        }
        for chunk in self.chunk_order.clone() {
            if !self.chunk_data.contains_key(&chunk) {
                return Err(Error::new(ErrorKind::NotFound, "Chunk not found"));
            }
            self.current_chunk = self.chunk_data.get(&chunk).expect("Chunk not found").clone();
            self.container.seek(SeekFrom::Start(
                self.current_chunk.start_offset,
            ))?;
            let mut deserialized_chunk = true;
            let chunk_name = chunk.clone();
            match chunk.to_str().expect("Chunk name is not valid UTF-8") {
                "GEN8" => {
                    deserialize_chunk!(chunk, ChunkGEN8);
                }
                "OPTN" => {
                    deserialize_chunk!(chunk, ChunkOPTN);
                }
                "LANG" => {
                    deserialize_chunk!(chunk, ChunkLANG);
                }
                "EXTN" => {
                    deserialize_chunk!(chunk, ChunkEXTN);
                }
                "SOND" => {
                    deserialize_chunk!(chunk, ChunkSOND);
                }
                "AGRP" => {
                    deserialize_chunk!(chunk, ChunkAGRP);
                }
                "SPRT" => {
                    deserialize_chunk!(chunk, ChunkSPRT);
                }
                "BGND" => {
                    deserialize_chunk!(chunk, ChunkBGND);
                }
                "PATH" => {
                    deserialize_chunk!(chunk, ChunkPATH);
                }
                "SCPT" => {
                    deserialize_chunk!(chunk, ChunkSCPT);
                }
                "GLOB" => {
                    deserialize_chunk!(chunk, ChunkGLOB);
                }
                "SHDR" => {
                    deserialize_chunk!(chunk, ChunkSHDR);
                }
                "FONT" => {
                    deserialize_chunk!(chunk, ChunkFONT);
                }
                "TMLN" => {
                    deserialize_chunk!(chunk, ChunkTMLN);
                }
                "OBJT" => {
                    deserialize_chunk!(chunk, ChunkOBJT);
                }
                "FEDS" => {
                    deserialize_chunk!(chunk, ChunkFEDS);
                }
                "ACRV" => {
                    deserialize_chunk!(chunk, ChunkACRV);
                }
                "SEQN" => {
                    deserialize_chunk!(chunk, ChunkSEQN);
                }
                _ => {
                    error!("No deserializer for chunk: {}", chunk);
                    deserialized_chunk = false;
                }
            }
            if deserialized_chunk {
                info!("Deserialized chunk: {}", chunk_name);
            }
        }
        Ok(())
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
    
    pub fn pad(&mut self, alignment: i64) -> Result<()> {
        let r = self.stream_position()? as i64 % alignment;
        if r != 0 {
            self.seek_relative(alignment - r)?;
        }
        Ok(())
    }

    pub fn pad_check_byte(&mut self, alignment: i64, byte: u8) -> Result<()> {
        while self.stream_position()? as i64 % alignment != 0 {
            if self.read_u8()? != byte {
                return Err(Error::new(ErrorKind::InvalidData, "Invalid padding byte"));
            }
        }
        Ok(())
    }
    
    pub fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.container.read(buf)
    }
    
    pub fn read_exact(&mut self, buf: &mut [u8]) -> Result<()> {
        self.container.read_exact(buf)
    }

    pub fn read_pointer_object_ext<P: Serialize>(&mut self, ptr: u64, return_after: bool) -> P {
        if ptr == 0 {
            panic!("Invalid (null) pointer.");
        }
        let return_to = self.container.stream_position().expect("Failed to get stram_position");
        self.container.seek(SeekFrom::Start(ptr)).expect("Failed to seek to pointer");
        let result = P::deserialize(self);
        if return_after {
            self.container.seek(SeekFrom::Start(return_to)).expect("Failed to seek back");
        }
        result
    }

    pub fn read_pointer_object<P: Serialize>(&mut self) -> P {
        let ptr = self.read_u32().expect("Failed to get object pointer");
        self.read_pointer_object_ext::<P>(ptr as _, true)
    }
    
    pub fn read_bool(&mut self) -> Result<bool> {
        let mut buf = [0; 1];
        self.container.read_exact(&mut buf)?;
        Ok(buf[0] != 0)
    }

    pub fn read_wide_bool(&mut self) -> Result<bool> {
        let mut buf = [0; 4];
        self.container.read_exact(&mut buf)?;
        Ok(u32::from_le_bytes(buf) != 0)
    }

    pub fn read_bytes<const S: usize>(&mut self) -> Result<[u8; S]> {
        let mut buf = [0u8; S];
        self.container.read_exact(&mut buf)?;
        Ok(buf)
    }

    pub fn read_pointer_string(&mut self) -> Result<BString> {
        let mut buf = [0; 4];
        self.container.read_exact(&mut buf)?;
        let pos = self.stream_position()?;
        let offset = u32::from_le_bytes(buf) as u64;
        if offset == 0 {
            return Err(Error::new(ErrorKind::InvalidData, "Pointer points to <null> value."));
        }
        self.container.seek(SeekFrom::Start(offset))?;
        let mut str = Vec::new();
        let mut i = self.container.read_u8()?;
        while i != 0 {
            str.push(i);
            i = self.container.read_u8()?;
        }
        self.container.seek(SeekFrom::Start(pos))?;
        Ok(BString::new(str))
    }

    pub fn read_pointer_string_safe(&mut self) -> Result<BString> {
        let mut buf = [0; 4];
        self.container.read_exact(&mut buf)?;
        let pos = self.stream_position()?;
        let offset = u32::from_le_bytes(buf) as u64;
        if offset == 0 {
            return Ok(BString::new(Vec::new()));
        }
        self.container.seek(SeekFrom::Start(offset))?;
        let mut str = Vec::new();
        let mut i = self.container.read_u8()?;
        while i != 0 {
            str.push(i);
            i = self.container.read_u8()?;
        }
        self.container.seek(SeekFrom::Start(pos))?;
        Ok(BString::new(str))
    }

    pub fn read_u8(&mut self) -> Result<u8> {
        self.container.read_u8()
    }

    pub fn read_u16(&mut self) -> Result<u16> {
        self.container.read_u16::<LittleEndian>()
    }

    pub fn read_u32(&mut self) -> Result<u32> {
        self.container.read_u32::<LittleEndian>()
    }

    pub fn read_u64(&mut self) -> Result<u64> {
        self.container.read_u64::<LittleEndian>()
    }

    pub fn read_u128(&mut self) -> Result<u128> {
        self.container.read_u128::<LittleEndian>()
    }

    pub fn read_i8(&mut self) -> Result<i8> {
        self.container.read_i8()
    }

    pub fn read_i16(&mut self) -> Result<i16> {
        self.container.read_i16::<LittleEndian>()
    }

    pub fn read_i32(&mut self) -> Result<i32> {
        self.container.read_i32::<LittleEndian>()
    }

    pub fn read_i64(&mut self) -> Result<i64> {
        self.container.read_i64::<LittleEndian>()
    }

    pub fn read_i128(&mut self) -> Result<i128> {
        self.container.read_i128::<LittleEndian>()
    }

    pub fn read_f32(&mut self) -> Result<f32> {
        self.container.read_f32::<LittleEndian>()
    }

    pub fn read_f64(&mut self) -> Result<f64> {
        self.container.read_f64::<LittleEndian>()
    }
}
