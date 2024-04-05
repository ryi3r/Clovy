use super::{reader::Reader, serializing::Serialize, writer::Writer};
use byteorder::WriteBytesExt;
use std::{fmt::Write, io::{Read, Seek, SeekFrom, Result}, ops::Index};

#[derive(Clone)]
pub struct GMPointerList<T> {
    pub container: T,
    pub values: Vec<T>,
}

impl<T> Default for GMPointerList<T>
    where T: Serialize + Default,
{
    fn default() -> Self {
        Self {
            container: T::default(),
            values: Vec::new(),
        }
    }
}

// (Reader, Entry Pointer, Current Entry, Entry Count)
type ReaderScriptBefore<R> = Box<dyn FnMut(&mut Reader<R>, u64, usize, usize) -> Result<()>>;
type ReaderScriptAfter<R> = Box<dyn FnMut(&mut Reader<R>, u64, usize, usize) -> Result<()>>;

// (Writer, Current Entry, Entry Count)
type WriterScriptBefore<W> = Box<dyn FnMut(&mut Writer<W>, usize, usize) -> Result<()>>;
type WriterScriptAfter<W> = Box<dyn FnMut(&mut Writer<W>, usize, usize) -> Result<()>>;

impl<T> GMPointerList<T>
    where T: Serialize,
{
    pub fn deserialize<R>(&mut self, reader: &mut Reader<R>, mut script_before: Option<ReaderScriptBefore<R>>, mut script_after: Option<ReaderScriptAfter<R>>) -> Result<()>
        where R: Read + Seek,
    {
        let mut ptr = Vec::new();
        for _ in 0..reader.read_i32()? {
            ptr.push(reader.read_u32()?);
        }
        let size = ptr.len();
        for (index, ptr) in ptr.iter().enumerate() {
            if let Some(script) = script_before.as_mut() {
                script(reader, *ptr as _, index, size)?;
            }
            reader.seek(SeekFrom::Start(*ptr as _))?;
            self.values.push(T::deserialize(reader)?);
            if let Some(script) = script_after.as_mut() {
                script(reader, *ptr as _, index, size)?;
            }
        }

        Ok(())
    }

    pub fn serialize<W>(&self, writer: &mut Writer<W>, mut script_before: Option<WriterScriptBefore<W>>, mut script_after: Option<WriterScriptAfter<W>>) -> Result<()>
    where
        W: Write + WriteBytesExt + Seek,
    {
        writer.write_u32(self.values.len() as _)?;
        let offset = writer.stream_position()?;
        for _ in 0..self.values.len() {
            writer.write_u32(0)?;
        }
        let size = self.values.len();
        for (index, value) in self.values.iter().enumerate() {
            if let Some(script) = script_before.as_mut() {
                script(writer, index, size)?;
            }
            let current_offset = writer.stream_position()?;
            writer.seek(SeekFrom::Start(offset + (index * 4) as u64))?;
            writer.write_u32(current_offset as _)?;
            writer.seek(SeekFrom::Start(current_offset))?;
            T::serialize(value, writer)?;
            if let Some(script) = script_after.as_mut() {
                script(writer, index, size)?;
            }
        }
        
        Ok(())
    }

    pub fn clear(&mut self) {
        self.values.clear();
    }

    pub fn push(&mut self, value: T) {
        self.values.push(value);
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }
}

impl<T> Serialize for GMPointerList<T>
    where T: Serialize + Default,
{
    fn deserialize<R>(reader: &mut Reader<R>) -> Result<Self>
            where R: Read + Seek {
        let mut list = Self {
            container: T::default(),
            values: Vec::new(),
        };
        list.deserialize(reader, None, None)?;

        Ok(list)
    }

    fn serialize<W>(chunk: &Self, writer: &mut Writer<W>) -> Result<()>
            where W: Write + WriteBytesExt + Seek {
        chunk.serialize(writer, None, None)
    }
}

impl<T> Index<usize> for GMPointerList<T> {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        &self.values[index]
    }
}

#[derive(Clone)]
pub struct GMSimpleList<T> {
    pub container: T,
    pub values: Vec<T>,
}

impl<T> Default for GMSimpleList<T>
    where T: Serialize + Default,
{
    fn default() -> Self {
        Self {
            container: T::default(),
            values: Vec::new(),
        }
    }
}

impl<T> GMSimpleList<T>
    where T: Serialize,
{
    pub fn deserialize<R>(&mut self, reader: &mut Reader<R>, mut script_before: Option<ReaderScriptBefore<R>>, mut script_after: Option<ReaderScriptAfter<R>>) -> Result<()>
        where R: Read + Seek,
    {
        let size = reader.read_i32()?;
        for index in 0..size {
            let pos = reader.stream_position()?;
            if let Some(script) = script_before.as_mut() {
                script(reader, pos, index as _, size as _)?;
            }
            self.values.push(T::deserialize(reader)?);
            if let Some(script) = script_after.as_mut() {
                script(reader, pos, index as _, size as _)?;
            }
        }

        Ok(())
    }

    pub fn serialize<W>(&self, writer: &mut Writer<W>, mut script_before: Option<WriterScriptBefore<W>>, mut script_after: Option<WriterScriptAfter<W>>) -> Result<()>
        where W: Write + WriteBytesExt + Seek,
    {
        let size = self.values.len();
        writer.write_u32(self.values.len() as _)?;
        for (index, value) in self.values.iter().enumerate() {
            if let Some(script) = script_before.as_mut() {
                script(writer, index as _, size)?;
            }
            T::serialize(value, writer)?;
            if let Some(script) = script_after.as_mut() {
                script(writer, index as _, size)?;
            }
        }

        Ok(())
    }

    pub fn clear(&mut self) {
        self.values.clear();
    }

    pub fn push(&mut self, value: T) {
        self.values.push(value);
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }
}

impl<T> Index<usize> for GMSimpleList<T> {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        &self.values[index]
    }
}

impl<T> Serialize for GMSimpleList<T>
    where T: Serialize + Default,
{
    fn deserialize<R>(reader: &mut Reader<R>) -> Result<Self>
            where R: Read + Seek {
        let mut list = Self {
            container: T::default(),
            values: Vec::new(),
        };
        list.deserialize(reader, None, None)?;
        
        Ok(list)
    }

    fn serialize<W>(chunk: &Self, writer: &mut Writer<W>) -> Result<()>
            where W: Write + WriteBytesExt + Seek {
        chunk.serialize(writer, None, None)?;

        Ok(())
    }
}
