use crate::core::{reader::Reader, serializing::Serialize, writer::Writer, lists::GMSimpleList};
use bitflags::bitflags;
use bstr::{BString, ByteSlice};
use byteorder::WriteBytesExt;
use tracing::{error, info, warn};
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
    pub keyframes: TrackKeyframes,
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

        chunk.broadcast_messages.deserialize(reader, None, None);
        chunk.tracks.deserialize(reader, None, None);

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
            chunk.channels.insert(channel, T::deserialize(reader));
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

        info!("{:?}", reader.stream_position());
        chunk.model_name = reader.read_pointer_string().expect("Failed to read model_name");
        chunk.name = reader.read_pointer_string_safe().expect("Failed to read name");
        chunk.built_in_name = reader.read_i32().expect("Failed to read built_in_name");
        chunk.traits = Trait::from_bits_retain(reader.read_i32().expect("Failed to read traits"));
        chunk.is_creation_track = reader.read_wide_bool().expect("Failed to read is_creation_track");

        let tag_count = reader.read_u32().expect("Failed to read tag_count");
        let owned_resource_count = reader.read_u32().expect("Failed to read tag_count");
        let track_count = reader.read_u32().expect("Failed to read tag_count");

        for _ in 0..tag_count {
            chunk.tags.push(reader.read_i32().expect("Failed to read tag"));
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
        info!("{:?}", chunk.model_name);
        match chunk.model_name.to_str().expect("String is not valid UTF-8") {
            "GMAudioTrack" => {
                chunk.keyframes = TrackKeyframes::Audio(AudioKeyframes::deserialize(reader));
            }
            "GMStringTrack" => {
                chunk.keyframes = TrackKeyframes::String(StringKeyframes::deserialize(reader));
            }
            "GMInstanceTrack" | "GMGraphicTrack" | "GMSequenceTrack" | "GMSpriteFramesTrack" | "GMBoolTrack" => {
                chunk.keyframes = TrackKeyframes::Default(DefaultKeyframes::deserialize(reader));
            }
            "GMParticleTrack" => {
                reader.version_info.set_version(2023, 2, 0, 0);
                chunk.keyframes = TrackKeyframes::Default(DefaultKeyframes::deserialize(reader));
            }
            "GMAssetTrack" => {
                error!("GMAssetTrack is not implemented. Please report this error!");
            }
            "GMRealTrack" | "GMColourTrack" => {
                chunk.keyframes = TrackKeyframes::Real(RealKeyframes::deserialize(reader));
            }
            "GMTextTrack" => {
                reader.version_info.set_version(2022, 2, 0, 0);
                chunk.keyframes = TrackKeyframes::Text(TextKeyframes::deserialize(reader));
            }
            _ => {
                error!("Unknown sequence {:?} model name", chunk.model_name);
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
            TrackKeyframes::Audio(audio) => {
                AudioKeyframes::serialize(audio, writer);
            }
            TrackKeyframes::String(string) => {
                StringKeyframes::serialize(string, writer);
            }
            TrackKeyframes::Default(default) => {
                DefaultKeyframes::serialize(default, writer);
            }
            TrackKeyframes::Real(real) => {
                RealKeyframes::serialize(real, writer);
            }
            TrackKeyframes::Text(text) => {
                TextKeyframes::serialize(text, writer);
            }
            _ => {
                error!("Unknown keyframe type");
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
pub enum TrackKeyframes {
    #[default]
    None,
    Default(DefaultKeyframes),
    Audio(AudioKeyframes),
    String(StringKeyframes),
    Real(RealKeyframes),
    Text(TextKeyframes),
}

#[derive(Default, Clone)]
pub struct DefaultKeyframes {
    pub data: u32,
}

impl Serialize for DefaultKeyframes {
    fn deserialize<R>(reader: &mut Reader<R>) -> Self
            where R: Read + Seek {
        let mut chunk = Self {
            ..Default::default()
        };

        //reader.pad_check_byte(4, 0).expect("Failed to pad reader");
        chunk.data = reader.read_u32().expect("Failed to write data");

        chunk
    }

    fn serialize<W>(chunk: &Self, writer: &mut Writer<W>)
            where W: Write + WriteBytesExt + Seek {
        //writer.pad_check_byte(4, 0).expect("Failed to pad writer");
        writer.write_u32(chunk.data).expect("Failed to write data");
    }
}

#[derive(Default, Clone)]
pub struct AudioKeyframes {
    // TODO: Replace this with the actual data
    pub data: u32, // Pointer -> SOND Chunk Resource ID
    pub mode: i32,
}

impl Serialize for AudioKeyframes {
    fn deserialize<R>(reader: &mut Reader<R>) -> Self
            where R: Read + Seek {
        let mut chunk = Self {
            ..Default::default()
        };

        //reader.pad_check_byte(4, 0).expect("Failed to pad reader");
        //chunk.data.deserialize(reader, None, None);
        chunk.data = reader.read_u32().expect("Failed to read data");
        if reader.read_u32().expect("Failed to read value (0)") != 0 {
            warn!("Expected 0 in Audio Keyframe (Offset: {})", reader.stream_position().expect("Failed to read stream position"));
        }
        chunk.mode = reader.read_i32().expect("Failed to read mode");

        chunk
    }

    fn serialize<W>(chunk: &Self, writer: &mut Writer<W>)
            where W: Write + WriteBytesExt + Seek {
        //writer.pad_check_byte(4, 0).expect("Failed to pad writer");
        //chunk.data.serialize(writer, None, None);
        writer.write_u32(chunk.data).expect("Failed to write data");
        writer.write_u32(0).expect("Failed to write value (0)");
        writer.write_i32(chunk.mode).expect("Failed to write value");
    }
}

#[derive(Default, Clone)]
pub struct StringKeyframes {
    pub data: BString,
}

impl Serialize for StringKeyframes {
    fn deserialize<R>(reader: &mut Reader<R>) -> Self
            where R: Read + Seek {
        let mut chunk = Self {
            ..Default::default()
        };

        //reader.pad_check_byte(4, 0).expect("Failed to pad reader");
        //chunk.data.deserialize(reader, None, None);
        chunk.data = reader.read_pointer_string().expect("Failed to read data");

        chunk
    }

    fn serialize<W>(chunk: &Self, writer: &mut Writer<W>)
            where W: Write + WriteBytesExt + Seek {
        //writer.pad_check_byte(4, 0).expect("Failed to pad writer");
        //chunk.data.serialize(writer, None, None);
        writer.write_pointer_string(&chunk.data).expect("Failed to write data");
    }
}

#[derive(Default, Clone)]
pub struct RealKeyframes {
    pub interpolation: i32,
    pub list: GMSimpleList<Keyframe<RealData>>,
}

impl Serialize for RealKeyframes {
    fn deserialize<R>(reader: &mut Reader<R>) -> Self
            where R: Read + Seek {
        let mut chunk = Self {
            ..Default::default()
        };

        reader.pad_check_byte(4, 0).expect("Failed to pad reader");
        //chunk.data.deserialize(reader, None, None);
        chunk.interpolation = reader.read_i32().expect("Failed to read interpolation");
        chunk.list.deserialize(reader, None, None);

        chunk
    }

    fn serialize<W>(chunk: &Self, writer: &mut Writer<W>)
            where W: Write + WriteBytesExt + Seek {
        writer.pad_check_byte(4, 0).expect("Failed to pad writer");
        //chunk.data.serialize(writer, None, None);
        writer.write_i32(chunk.interpolation).expect("Failed to write interpolation");
        chunk.list.serialize(writer, None, None);
    }
}

#[derive(Default, Clone)]
pub struct RealData {
    pub value: f32,
    pub curve: CurveData,
}

impl Serialize for RealData {
    fn deserialize<R>(reader: &mut Reader<R>) -> Self
            where R: Read + Seek {
        let mut chunk = Self {
            ..Default::default()
        };

        chunk.value = reader.read_f32().expect("Failed to read value");
        chunk.curve = CurveData::deserialize(reader);

        chunk
    }
    
    fn serialize<W>(chunk: &Self, writer: &mut Writer<W>)
            where W: Write + WriteBytesExt + Seek {
        writer.write_f32(chunk.value).expect("Failed to write value");
        CurveData::serialize(&chunk.curve, writer);
    }
}

#[derive(Default, Clone)]
pub struct CurveData {
    pub is_curve_embedded: bool,
    pub embedded_animation_curve: Option<AnimationCurve>,
    pub animation_curve_id: Option<u32>,
}

impl Serialize for CurveData {
    fn deserialize<R>(reader: &mut Reader<R>) -> Self
            where R: Read + Seek {
        let mut chunk = Self {
            ..Default::default()
        };

        chunk.is_curve_embedded = reader.read_wide_bool().expect("Failed to read is_curve_embedded");
        if chunk.is_curve_embedded {
            if reader.read_i32().expect("Failed to read value (-1)") != -1 {
                warn!("Expected -1 on CurveData");
            }
            chunk.embedded_animation_curve = Some(AnimationCurve::deserialize(reader));
        } else {
            chunk.animation_curve_id = Some(reader.read_u32().expect("Failed to read animation_curve_id"));
        }

        chunk
    }

    fn serialize<W>(chunk: &Self, writer: &mut Writer<W>)
            where W: Write + WriteBytesExt + Seek {
        writer.write_wide_bool(chunk.is_curve_embedded).expect("Failed to write is_curve_embedded");
        if chunk.is_curve_embedded {
            writer.write_i32(-1).expect("Failed to write value (-1)");
            AnimationCurve::serialize(chunk.embedded_animation_curve.as_ref().expect("Expected EmbeddedAnimationCurve but found nothing."), writer);
        } else {
            writer.write_u32(chunk.animation_curve_id.expect("Expected AnimationCurveId (i32) but found nothing.")).expect("Failed to write AnimationCurveId");
        }
    }
}

#[derive(Default, Clone)]
pub struct TextKeyframes {
    pub text: BString,
    pub wrap: bool,
    pub alignment: AlignmentMagic,
    pub font_index: i32,
}

impl Serialize for TextKeyframes {
    fn deserialize<R>(reader: &mut Reader<R>) -> Self
            where R: Read + Seek {
        let mut chunk = Self {
            ..Default::default()
        };

        chunk.text = reader.read_pointer_string().expect("Failed to read text");
        chunk.wrap = reader.read_wide_bool().expect("Failed to read wrap");
        chunk.alignment.magic_number = reader.read_i32().expect("Failed to read alignment");
        chunk.font_index = reader.read_i32().expect("Failed to read font_index");

        chunk
    }
    
    fn serialize<W>(chunk: &Self, writer: &mut Writer<W>)
            where W: Write + WriteBytesExt + Seek {
        writer.write_pointer_string(&chunk.text).expect("Failed to write text");
        writer.write_wide_bool(chunk.wrap).expect("Failed to write wrap");
        writer.write_i32(chunk.alignment.magic_number).expect("Failed to write alignment");
        writer.write_i32(chunk.font_index).expect("Failed to write font_index");
    }
}

#[derive(Default, Copy, Clone, PartialEq)]
pub struct AlignmentMagic {
    pub magic_number: i32,
}

impl AlignmentMagic {
    pub fn set_vertical_alignment(&mut self, value: i32) {
        self.magic_number = (self.magic_number & 0xff) | (value & 0xff) << 8;
    }

    pub fn set_horizontal_alignment(&mut self, value: i32) {
        self.magic_number = (self.magic_number & !0xff) | (value & 0xff);
    }

    pub fn get_vertical_alignment(&self) -> i32 {
        (self.magic_number >> 8) & 0xff
    }

    pub fn get_horizontal_alignment(&self) -> i32 {
        self.magic_number & 0xff
    }
}