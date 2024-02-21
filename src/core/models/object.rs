use crate::core::{reader::Reader, serializing::Serialize, writer::Writer, lists::GMPointerList};
use bstr::BString;
use byteorder::WriteBytesExt;
use std::{fmt::Write, io::{Read, Seek}};
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
    fn deserialize<R>(reader: &mut Reader<R>) -> Self
        where R: Read + Seek,
    {
        let mut chunk = Self {
            ..Default::default()
        };

        chunk.name = reader.read_pointer_string().expect("Failed to read name");
        chunk.sprite_id = reader.read_i32().expect("Failed to read sprite_id");
        chunk.visible = reader.read_wide_bool().expect("Failed to read visible");
        if reader.version_info.is_version_at_least(2022, 5, 0, 0) {
            chunk.managed = reader.read_wide_bool().expect("Failed to read managed");
        }
        chunk.solid = reader.read_wide_bool().expect("Failed to read solid");
        chunk.depth = reader.read_i32().expect("Failed to read depth");
        chunk.persistent = reader.read_wide_bool().expect("Failed to read persistent");
        chunk.parent_object_id = reader.read_i32().expect("Failed to read parent_object_id");
        chunk.mask_sprite_id = reader.read_i32().expect("Failed to read mask_sprite_id");
        chunk.physics = PhysicsProperties::deserialize(reader);
        chunk.events.deserialize(reader, None, None);

        chunk
    }

    fn serialize<W>(chunk: &Self, writer: &mut Writer<W>)
        where W: Write + WriteBytesExt + Seek,
    {
        writer.write_pointer_string(&chunk.name).expect("Failed to write name");
        writer.write_i32(chunk.sprite_id).expect("Failed to write sprite_id");
        writer.write_wide_bool(chunk.visible).expect("Failed to write visible");
        if writer.version_info.is_version_at_least(2022, 5, 0, 0) {
            writer.write_wide_bool(chunk.managed).expect("Failed to write managed");
        }
        writer.write_wide_bool(chunk.solid).expect("Failed to write solid");
        writer.write_i32(chunk.depth).expect("Failed to write depth");
        writer.write_wide_bool(chunk.persistent).expect("Failed to write persistent");
        writer.write_i32(chunk.parent_object_id).expect("Failed to write parent_object_id");
        writer.write_i32(chunk.mask_sprite_id).expect("Failed to write mask_sprite_id");
        PhysicsProperties::serialize(&chunk.physics, writer);
        chunk.events.serialize(writer, None, None);
    }
}

impl Serialize for PhysicsProperties {
    fn deserialize<R>(reader: &mut Reader<R>) -> Self
        where R: Read + Seek,
    {
        let mut chunk = Self {
            ..Default::default()
        };

        chunk.is_enabled = reader.read_wide_bool().expect("Failed to read is_enabled");
        chunk.sensor = reader.read_wide_bool().expect("Failed to read sensor");
        chunk.shape = CollisionShape::from_bits_retain(reader.read_i32().expect("Failed to read shape"));
        chunk.density = reader.read_f32().expect("Failed to read density");
        chunk.restitution = reader.read_f32().expect("Failed to read restitution");
        chunk.group = reader.read_i32().expect("Failed to read group");
        chunk.linear_damping = reader.read_f32().expect("Failed to read linear_damping");
        chunk.angular_damping = reader.read_f32().expect("Failed to read angular_damping");
        let vertex_count = reader.read_i32().expect("Failed to read vertex_count");
        chunk.friction = reader.read_f32().expect("Failed to read friction");
        chunk.is_awake = reader.read_wide_bool().expect("Failed to read is_awake");
        chunk.is_kinematic = reader.read_wide_bool().expect("Failed to read is_kinematic");
        for _ in 0..vertex_count {
            chunk.vertices.push(PhysicsVertex::deserialize(reader));
        }

        chunk
    }

    fn serialize<W>(chunk: &Self, writer: &mut Writer<W>)
        where W: Write + WriteBytesExt + Seek,
    {
        writer.write_wide_bool(chunk.is_enabled).expect("Failed to write is_enabled");
        writer.write_wide_bool(chunk.sensor).expect("Failed to write sensor");
        writer.write_i32(chunk.shape.bits()).expect("Failed to write shape");
        writer.write_f32(chunk.density).expect("Failed to write density");
        writer.write_f32(chunk.restitution).expect("Failed to write restitution");
        writer.write_i32(chunk.group).expect("Failed to write group");
        writer.write_f32(chunk.linear_damping).expect("Failed to write linear_damping");
        writer.write_f32(chunk.angular_damping).expect("Failed to write angular_damping");
        writer.write_i32(chunk.vertices.len() as i32).expect("Failed to write vertex_count");
        writer.write_f32(chunk.friction).expect("Failed to write friction");
        writer.write_wide_bool(chunk.is_awake).expect("Failed to write is_awake");
        writer.write_wide_bool(chunk.is_kinematic).expect("Failed to write is_kinematic");
        for vertex in chunk.vertices.iter() {
            PhysicsVertex::serialize(vertex, writer);
        }
    }
}

impl Serialize for PhysicsVertex {
    fn deserialize<R>(reader: &mut Reader<R>) -> Self
        where R: Read + Seek,
    {
        let mut chunk = Self {
            ..Default::default()
        };

        chunk.x = reader.read_f32().expect("Failed to read x");
        chunk.y = reader.read_f32().expect("Failed to read y");

        chunk
    }

    fn serialize<W>(chunk: &Self, writer: &mut Writer<W>)
        where W: Write + WriteBytesExt + Seek,
    {
        writer.write_f32(chunk.x).expect("Failed to write x");
        writer.write_f32(chunk.y).expect("Failed to write y");
    }
}

impl Serialize for Event {
    fn deserialize<R>(reader: &mut Reader<R>) -> Self
        where R: Read + Seek,
    {
        let mut chunk = Self {
            ..Default::default()
        };

        chunk.subtype = reader.read_i32().expect("Failed to read subtype");
        chunk.actions.deserialize(reader, None, None);

        chunk
    }

    fn serialize<W>(chunk: &Self, writer: &mut Writer<W>)
        where W: Write + WriteBytesExt + Seek,
    {
        writer.write_i32(chunk.subtype).expect("Failed to write subtype");
        chunk.actions.serialize(writer, None, None);
    }
}

impl Serialize for Action {
    fn deserialize<R>(reader: &mut Reader<R>) -> Self
        where R: Read + Seek,
    {
        let mut chunk = Self {
            ..Default::default()
        };

        chunk.lib_id = reader.read_i32().expect("Failed to read lib_id");
        chunk.id = reader.read_i32().expect("Failed to read id");
        chunk.kind = reader.read_i32().expect("Failed to read kind");
        chunk.use_relative = reader.read_wide_bool().expect("Failed to read use_relative");
        chunk.is_question = reader.read_wide_bool().expect("Failed to read is_question");
        chunk.use_apply_to = reader.read_wide_bool().expect("Failed to read use_apply_to");
        chunk.exe_type = reader.read_i32().expect("Failed to read exe_type");
        chunk.name = reader.read_pointer_string_safe().expect("Failed to read name");
        chunk.code_id = reader.read_i32().expect("Failed to read code_id");
        chunk.argument_count = reader.read_i32().expect("Failed to read argument_count");
        chunk.who = reader.read_i32().expect("Failed to read who");
        chunk.relative = reader.read_wide_bool().expect("Failed to read relative");
        chunk.is_not = reader.read_wide_bool().expect("Failed to read is_not");
        chunk.unknown = reader.read_i32().expect("Failed to read unknown");

        chunk
    }

    fn serialize<W>(chunk: &Self, writer: &mut Writer<W>)
        where W: Write + WriteBytesExt + Seek,
    {
        writer.write_i32(chunk.lib_id).expect("Failed to write lib_id");
        writer.write_i32(chunk.id).expect("Failed to write id");
        writer.write_i32(chunk.kind).expect("Failed to write kind");
        writer.write_wide_bool(chunk.use_relative).expect("Failed to write use_relative");
        writer.write_wide_bool(chunk.is_question).expect("Failed to write is_question");
        writer.write_wide_bool(chunk.use_apply_to).expect("Failed to write use_apply_to");
        writer.write_i32(chunk.exe_type).expect("Failed to write exe_type");
        writer.write_pointer_string(&chunk.name).expect("Failed to write name");
        writer.write_i32(chunk.code_id).expect("Failed to write code_id");
        writer.write_i32(chunk.argument_count).expect("Failed to write argument_count");
        writer.write_i32(chunk.who).expect("Failed to write who");
        writer.write_wide_bool(chunk.relative).expect("Failed to write relative");
        writer.write_wide_bool(chunk.is_not).expect("Failed to write is_not");
        writer.write_i32(chunk.unknown).expect("Failed to write unknown");
    }
}
