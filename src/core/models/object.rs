use crate::core::{reader::Reader, serializing::Serialize, writer::Writer, lists::GMPointerList};
use bstr::BString;
use byteorder::WriteBytesExt;
use std::{fmt::Write, io::{Read, Result, Seek}};
use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct CollisionShape: i32 {
        const Circle = 0;
        const Box = 1;
        const Custom = 2;
    }
}

#[derive(Default, Clone)]
pub struct Object {
    pub name: BString,
    pub sprite_id: i32,
    pub visible: bool,
    pub managed: bool,
    pub solid: bool,
    pub depth: i32,
    pub persistent: bool,
    pub parent_object_id: i32,
    pub mask_sprite_id: i32,
    pub physics: PhysicsProperties,
    pub events: GMPointerList<GMPointerList<Event>>,
}

#[derive(Default, Clone)]
pub struct PhysicsProperties {
    pub is_enabled: bool,
    pub sensor: bool,
    pub shape: CollisionShape,
    pub density: f32,
    pub restitution: f32,
    pub group: i32,
    pub linear_damping: f32,
    pub angular_damping: f32,
    pub vertices: Vec<PhysicsVertex>,
    pub friction: f32,
    pub is_awake: bool,
    pub is_kinematic: bool,
}

#[derive(Default, Clone)]
pub struct PhysicsVertex {
    pub x: f32,
    pub y: f32,
}

#[derive(Default, Clone)]
pub struct Event {
    pub subtype: i32,
    pub actions: GMPointerList<Action>,
}

#[derive(Default, Clone)]
pub struct Action {
    pub lib_id: i32,
    pub id: i32,
    pub kind: i32,
    pub use_relative: bool,
    pub is_question: bool,
    pub use_apply_to: bool,
    pub exe_type: i32,
    pub name: BString,
    pub code_id: i32,
    pub argument_count: i32,
    pub who: i32,
    pub relative: bool,
    pub is_not: bool,
    pub unknown: i32,
}

impl Default for CollisionShape {
    fn default() -> Self {
        Self::Box
    }
}

impl Serialize for Object {
    fn deserialize<R>(reader: &mut Reader<R>) -> Result<Self>
        where R: Read + Seek,
    {
        let mut chunk = Self {
            ..Default::default()
        };

        chunk.name = reader.read_pointer_string()?;
        chunk.sprite_id = reader.read_i32()?;
        chunk.visible = reader.read_wide_bool()?;
        if reader.version_info.is_version_at_least(2022, 5, 0, 0) {
            chunk.managed = reader.read_wide_bool()?;
        }
        chunk.solid = reader.read_wide_bool()?;
        chunk.depth = reader.read_i32()?;
        chunk.persistent = reader.read_wide_bool()?;
        chunk.parent_object_id = reader.read_i32()?;
        chunk.mask_sprite_id = reader.read_i32()?;
        chunk.physics = PhysicsProperties::deserialize(reader)?;
        chunk.events.deserialize(reader, None, None)?;

        Ok(chunk)
    }

    fn serialize<W>(chunk: &Self, writer: &mut Writer<W>) -> Result<()>
        where W: Write + WriteBytesExt + Seek,
    {
        writer.write_pointer_string(&chunk.name)?;
        writer.write_i32(chunk.sprite_id)?;
        writer.write_wide_bool(chunk.visible)?;
        if writer.version_info.is_version_at_least(2022, 5, 0, 0) {
            writer.write_wide_bool(chunk.managed)?;
        }
        writer.write_wide_bool(chunk.solid)?;
        writer.write_i32(chunk.depth)?;
        writer.write_wide_bool(chunk.persistent)?;
        writer.write_i32(chunk.parent_object_id)?;
        writer.write_i32(chunk.mask_sprite_id)?;
        PhysicsProperties::serialize(&chunk.physics, writer)?;
        chunk.events.serialize(writer, None, None)?;

        Ok(())
    }
}

impl Serialize for PhysicsProperties {
    fn deserialize<R>(reader: &mut Reader<R>) -> Result<Self>
        where R: Read + Seek,
    {
        let mut chunk = Self {
            ..Default::default()
        };

        chunk.is_enabled = reader.read_wide_bool()?;
        chunk.sensor = reader.read_wide_bool()?;
        chunk.shape = CollisionShape::from_bits_retain(reader.read_i32()?);
        chunk.density = reader.read_f32()?;
        chunk.restitution = reader.read_f32()?;
        chunk.group = reader.read_i32()?;
        chunk.linear_damping = reader.read_f32()?;
        chunk.angular_damping = reader.read_f32()?;
        let vertex_count = reader.read_i32()?;
        chunk.friction = reader.read_f32()?;
        chunk.is_awake = reader.read_wide_bool()?;
        chunk.is_kinematic = reader.read_wide_bool()?;
        for _ in 0..vertex_count {
            chunk.vertices.push(PhysicsVertex::deserialize(reader)?);
        }

        Ok(chunk)
    }

    fn serialize<W>(chunk: &Self, writer: &mut Writer<W>) -> Result<()>
        where W: Write + WriteBytesExt + Seek,
    {
        writer.write_wide_bool(chunk.is_enabled)?;
        writer.write_wide_bool(chunk.sensor)?;
        writer.write_i32(chunk.shape.bits())?;
        writer.write_f32(chunk.density)?;
        writer.write_f32(chunk.restitution)?;
        writer.write_i32(chunk.group)?;
        writer.write_f32(chunk.linear_damping)?;
        writer.write_f32(chunk.angular_damping)?;
        writer.write_i32(chunk.vertices.len() as i32)?;
        writer.write_f32(chunk.friction)?;
        writer.write_wide_bool(chunk.is_awake)?;
        writer.write_wide_bool(chunk.is_kinematic)?;
        for vertex in chunk.vertices.iter() {
            PhysicsVertex::serialize(vertex, writer)?;
        }

        Ok(())
    }
}

impl Serialize for PhysicsVertex {
    fn deserialize<R>(reader: &mut Reader<R>) -> Result<Self>
        where R: Read + Seek,
    {
        let mut chunk = Self {
            ..Default::default()
        };

        chunk.x = reader.read_f32()?;
        chunk.y = reader.read_f32()?;

        Ok(chunk)
    }

    fn serialize<W>(chunk: &Self, writer: &mut Writer<W>) -> Result<()>
        where W: Write + WriteBytesExt + Seek,
    {
        writer.write_f32(chunk.x)?;
        writer.write_f32(chunk.y)?;

        Ok(())
    }
}

impl Serialize for Event {
    fn deserialize<R>(reader: &mut Reader<R>) -> Result<Self>
        where R: Read + Seek,
    {
        let mut chunk = Self {
            ..Default::default()
        };

        chunk.subtype = reader.read_i32()?;
        chunk.actions.deserialize(reader, None, None)?;

        Ok(chunk)
    }

    fn serialize<W>(chunk: &Self, writer: &mut Writer<W>) -> Result<()>
        where W: Write + WriteBytesExt + Seek,
    {
        writer.write_i32(chunk.subtype)?;
        chunk.actions.serialize(writer, None, None)?;

        Ok(())
    }
}

impl Serialize for Action {
    fn deserialize<R>(reader: &mut Reader<R>) -> Result<Self>
        where R: Read + Seek,
    {
        let mut chunk = Self {
            ..Default::default()
        };

        chunk.lib_id = reader.read_i32()?;
        chunk.id = reader.read_i32()?;
        chunk.kind = reader.read_i32()?;
        chunk.use_relative = reader.read_wide_bool()?;
        chunk.is_question = reader.read_wide_bool()?;
        chunk.use_apply_to = reader.read_wide_bool()?;
        chunk.exe_type = reader.read_i32()?;
        chunk.name = reader.read_pointer_string_safe()?;
        chunk.code_id = reader.read_i32()?;
        chunk.argument_count = reader.read_i32()?;
        chunk.who = reader.read_i32()?;
        chunk.relative = reader.read_wide_bool()?;
        chunk.is_not = reader.read_wide_bool()?;
        chunk.unknown = reader.read_i32()?;

        Ok(chunk)
    }

    fn serialize<W>(chunk: &Self, writer: &mut Writer<W>) -> Result<()>
        where W: Write + WriteBytesExt + Seek,
    {
        writer.write_i32(chunk.lib_id)?;
        writer.write_i32(chunk.id)?;
        writer.write_i32(chunk.kind)?;
        writer.write_wide_bool(chunk.use_relative)?;
        writer.write_wide_bool(chunk.is_question)?;
        writer.write_wide_bool(chunk.use_apply_to)?;
        writer.write_i32(chunk.exe_type)?;
        writer.write_pointer_string(&chunk.name)?;
        writer.write_i32(chunk.code_id)?;
        writer.write_i32(chunk.argument_count)?;
        writer.write_i32(chunk.who)?;
        writer.write_wide_bool(chunk.relative)?;
        writer.write_wide_bool(chunk.is_not)?;
        writer.write_i32(chunk.unknown)?;

        Ok(())
    }
}
