use crate::core::{reader::Reader, serializing::Serialize, writer::Writer, lists::GMPointerList};
use bstr::BString;
use byteorder::WriteBytesExt;
use bitflags::bitflags;
use std::{fmt::Write, io::{Read, Seek}};

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ExtensionKind: i32 {
        const Unknown0 = 0;
        const Dll = 1;
        const Gml = 2;
        const Unknown3 = 3;
        const Generic = 4;
        const Js = 5;
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ExtensionValueType: i32 {
        const String = 1;
        const Double = 2;
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct OptionKind: i32 {
        const Boolean = 0;
        const String = 1;
        const Double = 2;
    }
}

#[derive(Default, Clone)]
pub struct Extension {
    pub folder_name: BString,
    pub name: BString,
    pub version: BString,
    pub class_name: BString,
    pub files: GMPointerList<ExtensionFile>,
    pub options: GMPointerList<ExtensionOption>,
    pub guid: Option<[u8; 16]>,
}

#[derive(Clone)]
pub struct ExtensionFile {
    pub filename: BString,
    pub final_function: BString,
    pub initial_function: BString,
    pub kind: ExtensionKind,
    pub functions: GMPointerList<ExtensionFunction>,
}

#[derive(Clone)]
pub struct ExtensionFunction {
    pub name: BString,
    pub id: i32,
    pub kind: i32,
    pub return_type: ExtensionValueType,
    pub external_name: BString,
    pub argument_types: Vec<ExtensionValueType>,
}

impl Default for ExtensionValueType {
    fn default() -> Self {
        Self::Double
    }
}

impl Default for ExtensionFile {
    fn default() -> Self {
        Self {
            filename: BString::default(),
            final_function: BString::default(),
            initial_function: BString::default(),
            kind: ExtensionKind::Gml,
            functions: GMPointerList::default(),
        }
    }
}

impl Default for ExtensionFunction {
    fn default() -> Self {
        Self {
            name: BString::default(),
            id: 0,
            kind: 0,
            return_type: ExtensionValueType::Double,
            external_name: BString::default(),
            argument_types: Vec::new(),
        }
    }
}

#[derive(Clone)]
pub struct ExtensionOption {
    pub name: BString,
    pub value: BString,
    pub kind: OptionKind,
}

impl Default for ExtensionOption {
    fn default() -> Self {
        Self {
            name: BString::default(),
            value: BString::default(),
            kind: OptionKind::Double,
        }
    }
}

impl Serialize for Extension {
    fn deserialize<R>(reader: &mut Reader<R>) -> Self
        where R: Read + Seek,
    {
        let mut chunk = Self {
            ..Default::default()
        };
        chunk.folder_name = reader.read_pointer_string().expect("Failed to read folder_name");
        chunk.name = reader.read_pointer_string().expect("Failed to read name");
        
        if reader.version_info.is_version_at_least(2023, 4, 0, 0) {
            chunk.version = reader.read_pointer_string().expect("Failed to read version");
        }
        chunk.class_name = reader.read_pointer_string().expect("Failed to read class_name");

        if reader.version_info.is_version_at_least(2022, 6, 0, 0) {
            chunk.files = reader.read_pointer_object::<GMPointerList<ExtensionFile>>();
            chunk.options = reader.read_pointer_object::<GMPointerList<ExtensionOption>>();
        } else {
            chunk.files.deserialize(reader, None, None);
        }

        chunk
    }

    fn serialize<W>(chunk: &Self, writer: &mut Writer<W>)
        where W: Write + WriteBytesExt + Seek,
    {
        writer.write_pointer_string(&chunk.folder_name).expect("Failed to write folder_name");
        writer.write_pointer_string(&chunk.name).expect("Failed to write name");

        if writer.version_info.is_version_at_least(2023, 4, 0, 0) {
            writer.write_pointer_string(&chunk.version).expect("Failed to write version");
        }
        writer.write_pointer_string(&chunk.class_name).expect("Failed to write class_name");

        if writer.version_info.is_version_at_least(2022, 6, 0, 0) {
            writer.write_pointer_object(&chunk.files).expect("Failed to write files");
            writer.write_pointer_object(&chunk.options).expect("Failed to write options");
        } else {
            chunk.files.serialize(writer, None, None);
        }
    }
}

impl Serialize for ExtensionFile {
    fn deserialize<R>(reader: &mut Reader<R>) -> Self
        where R: Read + Seek,
    {
        let mut chunk = Self {
            ..Default::default()
        };

        chunk.filename = reader.read_pointer_string().expect("Failed to read filename");
        chunk.final_function = reader.read_pointer_string().expect("Failed to read final_function");
        chunk.initial_function = reader.read_pointer_string().expect("Failed to read initial_function");
        chunk.kind = ExtensionKind::from_bits_retain(reader.read_i32().expect("Failed to read kind"));
        chunk.functions.deserialize(reader, None, None);

        chunk
    }

    fn serialize<W>(chunk: &Self, writer: &mut Writer<W>)
        where W: Write + WriteBytesExt + Seek,
    {
        writer.write_pointer_string(&chunk.filename).expect("Failed to write filename");
        writer.write_pointer_string(&chunk.final_function).expect("Failed to write final_function");
        writer.write_pointer_string(&chunk.initial_function).expect("Failed to write initial_function");
        writer.write_i32(chunk.kind.bits()).expect("Failed to write ID");
        chunk.functions.serialize(writer, None, None);
    }
}

impl Serialize for ExtensionFunction {
    fn deserialize<R>(reader: &mut Reader<R>) -> Self
        where R: Read + Seek,
    {
        let mut chunk = Self {
            ..Default::default()
        };

        chunk.name = reader.read_pointer_string().expect("Failed to read name");
        chunk.id = reader.read_i32().expect("Failed to read ID");
        chunk.kind = reader.read_i32().expect("Failed to read kind");
        chunk.return_type = ExtensionValueType::from_bits_retain(reader.read_i32().expect("Failed to read return_type"));
        chunk.external_name = reader.read_pointer_string().expect("Failed to read external_name");
        for _ in 0..reader.read_u32().expect("Failed to read argument_types length") {
            chunk.argument_types.push(ExtensionValueType::from_bits_retain(reader.read_i32().expect("Failed to read argument_types")));
        }

        chunk
    }

    fn serialize<W>(chunk: &Self, writer: &mut Writer<W>)
        where W: Write + WriteBytesExt + Seek,
    {
        writer.write_pointer_string(&chunk.name).expect("Failed to write name");
        writer.write_i32(chunk.id).expect("Failed to write ID");
        writer.write_i32(chunk.kind).expect("Failed to write kind");
        writer.write_i32(chunk.return_type.bits()).expect("Failed to write return_type");
        writer.write_pointer_string(&chunk.external_name).expect("Failed to write external_name");
        writer.write_u32(chunk.argument_types.len() as u32).expect("Failed to write argument_types length");
        for argument_type in chunk.argument_types.iter() {
            writer.write_i32(argument_type.bits()).expect("Failed to write argument_types");
        }
    }
}

impl Serialize for ExtensionOption {
    fn deserialize<R>(reader: &mut Reader<R>) -> Self
        where R: Read + Seek,
    {
        let mut chunk = Self {
            ..Default::default()
        };

        chunk.name = reader.read_pointer_string().expect("Failed to read name");
        chunk.value = reader.read_pointer_string().expect("Failed to read value");
        chunk.kind = OptionKind::from_bits_retain(reader.read_i32().expect("Failed to read kind"));

        chunk
    }

    fn serialize<W>(chunk: &Self, writer: &mut Writer<W>)
        where W: Write + WriteBytesExt + Seek,
    {
        writer.write_pointer_string(&chunk.name).expect("Failed to write name");
        writer.write_pointer_string(&chunk.value).expect("Failed to write value");
        writer.write_i32(chunk.kind.bits()).expect("Failed to write kind");
    }
}