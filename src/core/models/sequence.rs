use crate::core::{reader::Reader, serializing::Serialize, writer::Writer, lists::GMSimpleList};
use bitflags::bitflags;
use bstr::{BString, ByteSlice};
use byteorder::WriteBytesExt;
use tracing::{warn, info};
use std::{fmt::Write, io::{Read, Seek}};
use integer_hasher::IntMap;
use super::{sprite::AnimSpeedType, animation_curve::AnimationCurve};

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct PlaybackType: i32 {
        const Oneshot = 0;
        const Loop = 1;
        const Pingpong = 2;
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Trait: i32 {
        const Unknown1 = 0;
        const Unknown2 = 1;
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Interpolation: i32 {
        const None = 0;
        const Linear = 1;
    }
}

impl Default for PlaybackType {
    fn default() -> Self {
        Self::Oneshot
    }
}

impl Default for Trait {
    fn default() -> Self {
        Self::Unknown1
    }
}

impl Default for Interpolation {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Default, Clone)]
pub struct Sequence {
    pub name: BString,
    pub playback_type: PlaybackType,
    pub playback_speed: f32,
    pub playback_speed_type: AnimSpeedType,
    pub length: f32,
    pub origin_x: i32,
    pub origin_y: i32,
    pub volume: f32,
    pub broadcast_messages: GMSimpleList<Keyframe<BroadcastMessage>>,
    pub tracks: GMSimpleList<Track>,
    pub function_ids: IntMap<i32, BString>,
    pub moments: GMSimpleList<Keyframe<Moment>>,
}

#[derive(Default, Clone)]
pub struct Keyframe<T>
    where T: Serialize + Default,
{
    pub key: f32,
    pub length: f32,
    pub stretch: bool,
    pub disabled: bool,
    pub channels: IntMap<i32, T>,
}

#[derive(Default, Clone)]
pub struct BroadcastMessage {
    pub messages: Vec<BString>,
}

#[derive(Default, Clone)]
pub struct Track {
    pub model_name: BString,
    pub name: BString,
    pub built_in_name: i32,
    pub traits: Trait,
    pub is_creation_track: bool,
    pub tags: Vec<i32>,
    pub tracks: Vec<Track>,
    pub keyframes: i32,
    pub owned_resources: Vec<OwnedResources>,
    pub owned_resource_types: Vec<BString>,
}

#[derive(Clone)]
pub enum OwnedResources {
    None,
    AnimCurve(AnimationCurve),
}

impl Default for OwnedResources {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Default, Clone)]
pub struct Moment {
    pub internal_count: i32,
    pub event: BString,
}

impl Serialize for Sequence {
    fn deserialize<R>(reader: &mut Reader<R>) -> Self
        where R: Read + Seek,
    {
        let mut chunk = Self {
            ..Default::default()
        };

        chunk.name = reader.read_pointer_string().expect("Failed to read name");
        chunk.playback_type = PlaybackType::from_bits_retain(reader.read_i32().expect("Failed to read playback_type"));
        chunk.playback_speed = reader.read_f32().expect("Failed to read playback_speed");
        chunk.playback_speed_type = AnimSpeedType::from_bits_retain(reader.read_i32().expect("Failed to read playback_speed_type"));
        chunk.length = reader.read_f32().expect("Failed to read length");
        chunk.origin_x = reader.read_i32().expect("Failed to read origin_x");
        chunk.origin_y = reader.read_i32().expect("Failed to read origin_y");
        chunk.volume = reader.read_f32().expect("Failed to read volume");

        info!("A");
        chunk.broadcast_messages.deserialize(reader, None, None);
        info!("B");
        chunk.tracks.deserialize(reader, None, None);
        info!("C");

        for _ in 0..reader.read_u32().expect("Failed to read function_ids count") {
            let key = reader.read_i32().expect("Failed to read function_id");
            chunk.function_ids.insert(key, reader.read_pointer_string().expect("Failed to read function_id"));
        }

        chunk.moments.deserialize(reader, None, None);

        chunk
    }

    fn serialize<W>(chunk: &Self, writer: &mut Writer<W>)
        where W: Write + WriteBytesExt + Seek,
    {
        writer.write_pointer_string(&chunk.name).expect("Failed to write name");
        writer.write_i32(chunk.playback_type.bits()).expect("Failed to write playback_type");
        writer.write_f32(chunk.playback_speed).expect("Failed to write playback_speed");
        writer.write_i32(chunk.playback_speed_type.bits()).expect("Failed to write playback_speed_type");
        writer.write_f32(chunk.length).expect("Failed to write length");
        writer.write_i32(chunk.origin_x).expect("Failed to write origin_x");
        writer.write_i32(chunk.origin_y).expect("Failed to write origin_y");
        writer.write_f32(chunk.volume).expect("Failed to write volume");

        chunk.broadcast_messages.serialize(writer, None, None);
        chunk.tracks.serialize(writer, None, None);

        writer.write_u32(chunk.function_ids.len() as u32).expect("Failed to write function_ids count");
        for (key, value) in chunk.function_ids.iter() {
            writer.write_i32(*key).expect("Failed to write function_id id");
            writer.write_pointer_string(value).expect("Failed to write function_id value");
        }

        chunk.moments.serialize(writer, None, None);
    }
}

impl<T> Serialize for Keyframe<T>
    where T: Serialize + Default,
{
    fn deserialize<R>(reader: &mut Reader<R>) -> Self
        where R: Read + Seek,
    {
        let mut chunk = Self {
            ..Default::default()
        };

        chunk.key = reader.read_f32().expect("Failed to read key");
        chunk.length = reader.read_f32().expect("Failed to read length");
        chunk.stretch = reader.read_bool().expect("Failed to read stretch");
        chunk.disabled = reader.read_bool().expect("Failed to read disabled");
        for _ in 0..reader.read_u32().expect("Failed to read channel count") {
            let channel = reader.read_i32().expect("Failed to read channel");
            let data = T::deserialize(reader);
            chunk.channels.insert(channel, data);
        }

        chunk
    }

    fn serialize<W>(chunk: &Self, writer: &mut Writer<W>)
        where W: Write + WriteBytesExt + Seek,
    {
        writer.write_f32(chunk.key).expect("Failed to write key");
        writer.write_f32(chunk.length).expect("Failed to write length");
        writer.write_bool(chunk.stretch).expect("Failed to write stretch");
        writer.write_bool(chunk.disabled).expect("Failed to write disabled");
        writer.write_u32(chunk.channels.len() as u32).expect("Failed to write channel count");
        for (channel, data) in chunk.channels.iter() {
            writer.write_i32(*channel).expect("Failed to write channel");
            T::serialize(data, writer);
        }
    }
}

impl Serialize for BroadcastMessage {
    fn deserialize<R>(reader: &mut Reader<R>) -> Self
            where R: Read + Seek {
        let mut chunk = Self {
            ..Default::default()
        };

        for _ in 0..reader.read_u32().expect("Failed to read message count") {
            chunk.messages.push(reader.read_pointer_string().expect("Failed to read message"));
        }

        chunk
    }

    fn serialize<W>(chunk: &Self, writer: &mut Writer<W>)
            where W: Write + WriteBytesExt + Seek {
        writer.write_u32(chunk.messages.len() as u32).expect("Failed to write message count");
        for message in chunk.messages.iter() {
            writer.write_pointer_string(message).expect("Failed to write message");
        }
    }
}

impl Serialize for Track {
    fn deserialize<R>(reader: &mut Reader<R>) -> Self
            where R: Read + Seek {
        let mut chunk = Self {
            ..Default::default()
        };

        chunk.model_name = reader.read_pointer_string().expect("Failed to read model_name");
        chunk.name = reader.read_pointer_string().expect("Failed to read name");
        chunk.built_in_name = reader.read_i32().expect("Failed to read built_in_name");
        chunk.traits = Trait::from_bits_retain(reader.read_i32().expect("Failed to read traits"));
        chunk.is_creation_track = reader.read_wide_bool().expect("Failed to read is_creation_track");

        let tag_count = reader.read_u32().expect("Failed to read tag_count");
        let owned_resource_count = reader.read_u32().expect("Failed to read tag_count");
        let track_count = reader.read_u32().expect("Failed to read tag_count");

        for _ in 0..tag_count {
            info!("D");
            chunk.tags.push(reader.read_i32().expect("Failed to read tag"));
            info!("E");
        }
        for _ in 0..owned_resource_count {
            let str = reader.read_pointer_string_safe().expect("Failed to read owned_resource");
            chunk.owned_resource_types.push(str.clone());
            info!("{str:?}");
            info!("{:?}", reader.stream_position());
            if str.to_str().expect("String is not valid UTF-8") == "GMAnimCurve" {
                chunk.owned_resources.push(OwnedResources::AnimCurve(AnimationCurve::deserialize(reader)));
            } else {
                warn!("Unknown resource type: {str:?}");
            }
        }
        for _ in 0..track_count {
            chunk.tracks.push(Track::deserialize(reader));
        }
        match chunk.model_name.to_str().expect("String is not valid UTF-8") {
            
            _ => {
                //panic!("Unknown sequence {:?} model name", chunk.model_name);
            }
        }

        chunk
    }

    fn serialize<W>(chunk: &Self, writer: &mut Writer<W>)
            where W: Write + WriteBytesExt + Seek {
        writer.write_pointer_string(&chunk.model_name).expect("Failed to write model_name");
        writer.write_pointer_string(&chunk.name).expect("Failed to write name");
        writer.write_i32(chunk.built_in_name).expect("Failed to write built_in_name");
        writer.write_i32(chunk.traits.bits()).expect("Failed to write traits");
        writer.write_wide_bool(chunk.is_creation_track).expect("Failed to write is_creation_track");

        writer.write_u32(chunk.tags.len() as u32).expect("Failed to write tag_count");
        writer.write_u32(chunk.owned_resources.len() as u32).expect("Failed to write tag_count");
        writer.write_u32(chunk.tracks.len() as u32).expect("Failed to write tag_count");

        for tag in chunk.tags.iter() {
            writer.write_i32(*tag).expect("Failed to write tag");
        }

        for owned_resource in chunk.owned_resources.iter() {
            match owned_resource {
                OwnedResources::None => {
                    panic!("No resource type???");
                }
                OwnedResources::AnimCurve(curve) => {
                    writer.write_pointer::<AnimationCurve>(curve.clone()).expect("Failed to write animation curve pointer");
                }
            }
        }

        for track in chunk.tracks.iter() {
            Track::serialize(track, writer);
        }

        match &chunk.keyframes {
            _ => {
                panic!("No keyframes???");
            }
        }

    }
}

impl Serialize for Moment {
    fn deserialize<R>(reader: &mut Reader<R>) -> Self
            where R: Read + Seek {
        let mut chunk = Self {
            ..Default::default()
        };

        chunk.internal_count = reader.read_i32().expect("Failed to read internal_count");
        if chunk.internal_count > 0 {
            chunk.event = reader.read_pointer_string().expect("Failed to read event");
        }

        chunk
    }

    fn serialize<W>(chunk: &Self, writer: &mut Writer<W>)
            where W: Write + WriteBytesExt + Seek {
        writer.write_i32(chunk.internal_count).expect("Failed to write internal_count");
        if chunk.internal_count > 0 {
            writer.write_pointer_string(&chunk.event).expect("Failed to write event");
        }
    }
}

#[derive(Default, Clone)]
pub struct TrackKeyframes<T>
    where T: Serialize + Default,
{
    pub data: GMSimpleList<Keyframe<T>>,
}

impl<T> Serialize for TrackKeyframes<T>
    where T: Serialize + Default,
{
    fn deserialize<R>(reader: &mut Reader<R>) -> Self
            where R: Read + Seek {
        let mut chunk = Self {
            ..Default::default()
        };

        reader.pad_check_byte(4, 0).expect("Failed to pad reader");
        chunk.data.deserialize(reader, None, None);

        chunk
    }

    fn serialize<W>(chunk: &Self, writer: &mut Writer<W>)
            where W: Write + WriteBytesExt + Seek {
        writer.pad_check_byte(4, 0).expect("Failed to pad writer");
        chunk.data.serialize(writer, None, None);
    }
}