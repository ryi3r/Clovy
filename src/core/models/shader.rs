use crate::core::{reader::Reader, serializing::Serialize, writer::Writer};
use bitflags::bitflags;
use bstr::BString;
use byteorder::WriteBytesExt;
use tracing::info;
use std::{fmt::Write, io::{Read, Result, Seek}};

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ShaderType: i32 {
        const GlslEs = 1;
        const Glel = 2;
        const Hlsl9 = 3;
        const Hlsl11 = 4;
        const Pssl = 5;
        const CgPsVita = 6;
        const CgPs3 = 7;
    }
}

#[derive(Clone)]
pub struct Shader {
    pub name: BString,
    pub kind: ShaderType,
    pub glsl_es_vertex: BString,
    pub glsl_es_fragment: BString,
    pub glsl_vertex: BString,
    pub glsl_fragment: BString,
    pub hlsl9_vertex: BString,
    pub hlsl9_fragment: BString,
    pub hlsl11_vertex_buffer: Vec<u8>,
    pub hlsl11_pixel_buffer: Vec<u8>,
    pub pssl_vertex_buffer: Vec<u8>,
    pub pssl_pixel_buffer: Vec<u8>,
    pub cg_psv_vertex_buffer: Vec<u8>,
    pub cg_psv_pixel_buffer: Vec<u8>,
    pub cg_ps3_vertex_buffer: Vec<u8>,
    pub cg_ps3_pixel_buffer: Vec<u8>,
    pub vertex_attributes: Vec<BString>,
    pub version: i32, // Default is 2
}

impl Default for Shader {
    fn default() -> Self {
        Self {
            name: BString::default(),
            kind: ShaderType::GlslEs,
            glsl_es_vertex: BString::default(),
            glsl_es_fragment: BString::default(),
            glsl_vertex: BString::default(),
            glsl_fragment: BString::default(),
            hlsl9_vertex: BString::default(),
            hlsl9_fragment: BString::default(),
            hlsl11_vertex_buffer: Vec::new(),
            hlsl11_pixel_buffer: Vec::new(),
            pssl_vertex_buffer: Vec::new(),
            pssl_pixel_buffer: Vec::new(),
            cg_psv_vertex_buffer: Vec::new(),
            cg_psv_pixel_buffer: Vec::new(),
            cg_ps3_vertex_buffer: Vec::new(),
            cg_ps3_pixel_buffer: Vec::new(),
            vertex_attributes: Vec::new(),
            version: 2,
        }
    }
}

impl Serialize for Shader {
    fn deserialize<R>(reader: &mut Reader<R>) -> Result<Self>
        where R: Read + Seek,
    {
        let mut chunk = Self {
            ..Default::default()
        };

        chunk.name = reader.read_pointer_string()?;
        chunk.kind = ShaderType::from_bits_retain(reader.read_i32()?);
        chunk.glsl_es_vertex = reader.read_pointer_string()?;
        chunk.glsl_es_fragment = reader.read_pointer_string()?;
        chunk.glsl_vertex = reader.read_pointer_string()?;
        chunk.glsl_fragment = reader.read_pointer_string()?;
        chunk.hlsl9_vertex = reader.read_pointer_string()?;
        chunk.hlsl9_fragment = reader.read_pointer_string()?;
        let ptr1 = reader.read_i32()?;
        info!("{:?}", ptr1);

        todo!("Not implemented");
        // TODO: UTY's team didn't code shaders so I'll look into it later with another game
        //chunk.hlsl11_vertex_buffer = reader.read_pointer_object(ptr1 as usize)?;
        //let ptr2 = reader.read_i32()?;
        //chunk.hlsl11_pixel_buffer = reader.read_pointer_object(ptr2 as usize)?;

        //chunk
    }

    fn serialize<W>(_chunk: &Self, _writer: &mut Writer<W>) -> Result<()>
        where W: Write + WriteBytesExt + Seek,
    {
        todo!("Not implemented");
    }
}
