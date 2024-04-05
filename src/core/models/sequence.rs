use crate::core::{reader::Reader, serializing::Serialize, writer::Writer, lists::GMSimpleList};
use bitflags::bitflags;
use bstr::{BString, ByteSlice};
use byteorder::WriteBytesExt;
use tracing::{error, info, warn};
use std::{fmt::Write, io::{Error, ErrorKind, Read, Result, Seek}};
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
    fn deserialize<R>(reader: &mut Reader<R>) -> Result<Self>
        where R: Read + Seek,
    {
        let mut chunk = Self {
            ..Default::default()
        };

        chunk.name = reader.read_pointer_string()?;
        chunk.playback_type = PlaybackType::from_bits_retain(reader.read_i32()?);
        chunk.playback_speed = reader.read_f32()?;
        chunk.playback_speed_type = AnimSpeedType::from_bits_retain(reader.read_i32()?);
        chunk.length = reader.read_f32()?;
        chunk.origin_x = reader.read_i32()?;
        chunk.origin_y = reader.read_i32()?;
        chunk.volume = reader.read_f32()?;

        chunk.broadcast_messages.deserialize(reader, None, None)?;
        chunk.tracks.deserialize(reader, None, None)?;

        for _ in 0..reader.read_u32()? {
            let key = reader.read_i32()?;
            chunk.function_ids.insert(key, reader.read_pointer_string()?);
        }

        chunk.moments.deserialize(reader, None, None)?;

        Ok(chunk)
    }

    fn serialize<W>(chunk: &Self, writer: &mut Writer<W>) -> Result<()>
        where W: Write + WriteBytesExt + Seek,
    {
        writer.write_pointer_string(&chunk.name)?;
        writer.write_i32(chunk.playback_type.bits())?;
        writer.write_f32(chunk.playback_speed)?;
        writer.write_i32(chunk.playback_speed_type.bits())?;
        writer.write_f32(chunk.length)?;
        writer.write_i32(chunk.origin_x)?;
        writer.write_i32(chunk.origin_y)?;
        writer.write_f32(chunk.volume)?;

        chunk.broadcast_messages.serialize(writer, None, None)?;
        chunk.tracks.serialize(writer, None, None)?;

        writer.write_u32(chunk.function_ids.len() as u32)?;
        for (key, value) in chunk.function_ids.iter() {
            writer.write_i32(*key)?;
            writer.write_pointer_string(value)?;
        }

        chunk.moments.serialize(writer, None, None)?;

        Ok(())
    }
}

impl<T> Serialize for Keyframe<T>
    where T: Serialize + Default,
{
    fn deserialize<R>(reader: &mut Reader<R>) -> Result<Self>
        where R: Read + Seek,
    {
        let mut chunk = Self {
            ..Default::default()
        };

        chunk.key = reader.read_f32()?;
        chunk.length = reader.read_f32()?;
        chunk.stretch = reader.read_bool()?;
        chunk.disabled = reader.read_bool()?;
        for _ in 0..reader.read_u32()? {
            let channel = reader.read_i32()?;
            chunk.channels.insert(channel, T::deserialize(reader)?);
        }

        Ok(chunk)
    }

    fn serialize<W>(chunk: &Self, writer: &mut Writer<W>) -> Result<()>
        where W: Write + WriteBytesExt + Seek,
    {
        writer.write_f32(chunk.key)?;
        writer.write_f32(chunk.length)?;
        writer.write_bool(chunk.stretch)?;
        writer.write_bool(chunk.disabled)?;
        writer.write_u32(chunk.channels.len() as u32)?;
        for (channel, data) in chunk.channels.iter() {
            writer.write_i32(*channel)?;
            T::serialize(data, writer)?;
        }

        Ok(())
    }
}

impl Serialize for BroadcastMessage {
    fn deserialize<R>(reader: &mut Reader<R>) -> Result<Self>
            where R: Read + Seek {
        let mut chunk = Self {
            ..Default::default()
        };

        for _ in 0..reader.read_u32()? {
            chunk.messages.push(reader.read_pointer_string()?);
        }

        Ok(chunk)
    }

    fn serialize<W>(chunk: &Self, writer: &mut Writer<W>) -> Result<()>
            where W: Write + WriteBytesExt + Seek {
        writer.write_u32(chunk.messages.len() as u32)?;
        for message in chunk.messages.iter() {
            writer.write_pointer_string(message)?;
        }

        Ok(())
    }
}

impl Serialize for Track {
    fn deserialize<R>(reader: &mut Reader<R>) -> Result<Self>
            where R: Read + Seek {
        let mut chunk = Self {
            ..Default::default()
        };

        info!("{:?}", reader.stream_position());
        chunk.model_name = reader.read_pointer_string()?;
        chunk.name = reader.read_pointer_string_safe()?;
        chunk.built_in_name = reader.read_i32()?;
        chunk.traits = Trait::from_bits_retain(reader.read_i32()?);
        chunk.is_creation_track = reader.read_wide_bool()?;

        let tag_count = reader.read_u32()?;
        let owned_resource_count = reader.read_u32()?;
        let track_count = reader.read_u32()?;

        for _ in 0..tag_count {
            chunk.tags.push(reader.read_i32()?);
        }
        for _ in 0..owned_resource_count {
            let str = reader.read_pointer_string_safe()?;
            chunk.owned_resource_types.push(str.clone());
            info!("{str:?}");
            info!("{:?}", reader.stream_position());
            if str.to_str() == Ok("GMAnimCurve") {
                chunk.owned_resources.push(OwnedResources::AnimCurve(AnimationCurve::deserialize(reader)?));
            } else {
                warn!("Unknown resource type: {str:?}");
            }
        }
        for _ in 0..track_count {
            chunk.tracks.push(Track::deserialize(reader)?);
        }
        info!("{:?}", chunk.model_name);
        match chunk.model_name.to_str() {
            Ok("GMAudioTrack") => {
                chunk.keyframes = TrackKeyframes::Audio(AudioKeyframes::deserialize(reader)?);
            }
            Ok("GMStringTrack") => {
                chunk.keyframes = TrackKeyframes::String(StringKeyframes::deserialize(reader)?);
            }
            Ok("GMInstanceTrack") | Ok("GMGraphicTrack") | Ok("GMSequenceTrack") | Ok("GMSpriteFramesTrack") | Ok("GMBoolTrack") => {
                chunk.keyframes = TrackKeyframes::Default(DefaultKeyframes::deserialize(reader)?);
            }
            Ok("GMParticleTrack") => {
                reader.version_info.set_version(2023, 2, 0, 0);
                chunk.keyframes = TrackKeyframes::Default(DefaultKeyframes::deserialize(reader)?);
            }
            Ok("GMAssetTrack") => {
                error!("GMAssetTrack is not implemented. Please report this error!");
            }
            Ok("GMRealTrack") | Ok("GMColourTrack") => {
                chunk.keyframes = TrackKeyframes::Real(RealKeyframes::deserialize(reader)?);
            }
            Ok("GMTextTrack") => {
                reader.version_info.set_version(2022, 2, 0, 0);
                chunk.keyframes = TrackKeyframes::Text(TextKeyframes::deserialize(reader)?);
            }
            _ => {
                error!("Unknown sequence {:?} model name", chunk.model_name);
            }
        }

        Ok(chunk)
    }

    fn serialize<W>(chunk: &Self, writer: &mut Writer<W>) -> Result<()>
            where W: Write + WriteBytesExt + Seek {
        writer.write_pointer_string(&chunk.model_name)?;
        writer.write_pointer_string(&chunk.name)?;
        writer.write_i32(chunk.built_in_name)?;
        writer.write_i32(chunk.traits.bits())?;
        writer.write_wide_bool(chunk.is_creation_track)?;

        writer.write_u32(chunk.tags.len() as u32)?;
        writer.write_u32(chunk.owned_resources.len() as u32)?;
        writer.write_u32(chunk.tracks.len() as u32)?;

        for tag in chunk.tags.iter() {
            writer.write_i32(*tag)?;
        }

        for owned_resource in chunk.owned_resources.iter() {
            match owned_resource {
                OwnedResources::None => {
                    panic!("No resource type???");
                }
                OwnedResources::AnimCurve(curve) => {
                    writer.write_pointer::<AnimationCurve>(curve.clone())?;
                }
            }
        }

        for track in chunk.tracks.iter() {
            Track::serialize(track, writer)?;
        }

        match &chunk.keyframes {
            TrackKeyframes::Audio(audio) => { AudioKeyframes::serialize(audio, writer)?; }
            TrackKeyframes::String(string) => { StringKeyframes::serialize(string, writer)?; }
            TrackKeyframes::Default(default) => { DefaultKeyframes::serialize(default, writer)?; }
            TrackKeyframes::Real(real) => { RealKeyframes::serialize(real, writer)?; }
            TrackKeyframes::Text(text) => { TextKeyframes::serialize(text, writer)?; }
            _ => {
                error!("Unknown keyframe type");
            }
        }

        Ok(())
    }
}

impl Serialize for Moment {
    fn deserialize<R>(reader: &mut Reader<R>) -> Result<Self>
            where R: Read + Seek {
        let mut chunk = Self {
            ..Default::default()
        };

        chunk.internal_count = reader.read_i32()?;
        if chunk.internal_count > 0 {
            chunk.event = reader.read_pointer_string()?;
        }

        Ok(chunk)
    }

    fn serialize<W>(chunk: &Self, writer: &mut Writer<W>) -> Result<()>
            where W: Write + WriteBytesExt + Seek {
        writer.write_i32(chunk.internal_count)?;
        if chunk.internal_count > 0 {
            writer.write_pointer_string(&chunk.event)?;
        }

        Ok(())
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
    fn deserialize<R>(reader: &mut Reader<R>) -> Result<Self>
            where R: Read + Seek {
        let mut chunk = Self {
            ..Default::default()
        };

        //reader.pad_check_byte(4, 0)?;
        chunk.data = reader.read_u32()?;

        Ok(chunk)
    }

    fn serialize<W>(chunk: &Self, writer: &mut Writer<W>) -> Result<()>
            where W: Write + WriteBytesExt + Seek {
        //writer.pad_check_byte(4, 0)?;
        writer.write_u32(chunk.data)?;

        Ok(())
    }
}

#[derive(Default, Clone)]
pub struct AudioKeyframes {
    // TODO: Replace this with the actual data
    pub data: u32, // Pointer -> SOND Chunk Resource ID
    pub mode: i32,
}

impl Serialize for AudioKeyframes {
    fn deserialize<R>(reader: &mut Reader<R>) -> Result<Self>
            where R: Read + Seek {
        let mut chunk = Self {
            ..Default::default()
        };

        //reader.pad_check_byte(4, 0)?;
        //chunk.data.deserialize(reader, None, None)?;
        chunk.data = reader.read_u32()?;
        if reader.read_u32()? != 0 {
            warn!("Expected 0 in Audio Keyframe (Offset: {})", reader.stream_position()?);
        }
        chunk.mode = reader.read_i32()?;

        Ok(chunk)
    }

    fn serialize<W>(chunk: &Self, writer: &mut Writer<W>) -> Result<()>
            where W: Write + WriteBytesExt + Seek {
        //writer.pad_check_byte(4, 0)?;
        //chunk.data.serialize(writer, None, None)?;
        writer.write_u32(chunk.data)?;
        writer.write_u32(0)?;
        writer.write_i32(chunk.mode)?;

        Ok(())
    }
}

#[derive(Default, Clone)]
pub struct StringKeyframes {
    pub data: BString,
}

impl Serialize for StringKeyframes {
    fn deserialize<R>(reader: &mut Reader<R>) -> Result<Self>
            where R: Read + Seek {
        let mut chunk = Self {
            ..Default::default()
        };

        //reader.pad_check_byte(4, 0).expect("Failed to pad reader");
        //chunk.data.deserialize(reader, None, None);
        chunk.data = reader.read_pointer_string().expect("Failed to read data");

        Ok(chunk)
    }

    fn serialize<W>(chunk: &Self, writer: &mut Writer<W>) -> Result<()>
            where W: Write + WriteBytesExt + Seek {
        //writer.pad_check_byte(4, 0)?;
        //chunk.data.serialize(writer, None, None)?;
        writer.write_pointer_string(&chunk.data)?;

        Ok(())
    }
}

#[derive(Default, Clone)]
pub struct RealKeyframes {
    pub interpolation: i32,
    pub list: GMSimpleList<Keyframe<RealData>>,
}

impl Serialize for RealKeyframes {
    fn deserialize<R>(reader: &mut Reader<R>) -> Result<Self>
            where R: Read + Seek {
        let mut chunk = Self {
            ..Default::default()
        };

        reader.pad_check_byte(4, 0).expect("Failed to pad reader");
        //chunk.data.deserialize(reader, None, None);
        chunk.interpolation = reader.read_i32().expect("Failed to read interpolation");
        chunk.list.deserialize(reader, None, None)?;

        Ok(chunk)
    }

    fn serialize<W>(chunk: &Self, writer: &mut Writer<W>) -> Result<()>
            where W: Write + WriteBytesExt + Seek {
        writer.pad_check_byte(4, 0).expect("Failed to pad writer");
        //chunk.data.serialize(writer, None, None);
        writer.write_i32(chunk.interpolation).expect("Failed to write interpolation");
        chunk.list.serialize(writer, None, None)?;

        Ok(())
    }
}

#[derive(Default, Clone)]
pub struct RealData {
    pub value: f32,
    pub curve: CurveData,
}

impl Serialize for RealData {
    fn deserialize<R>(reader: &mut Reader<R>) -> Result<Self>
            where R: Read + Seek {
        let mut chunk = Self {
            ..Default::default()
        };

        chunk.value = reader.read_f32()?;
        chunk.curve = CurveData::deserialize(reader)?;

        Ok(chunk)
    }
    
    fn serialize<W>(chunk: &Self, writer: &mut Writer<W>) -> Result<()>
            where W: Write + WriteBytesExt + Seek {
        writer.write_f32(chunk.value).expect("Failed to write value");
        CurveData::serialize(&chunk.curve, writer)?;

        Ok(())
    }
}

#[derive(Default, Clone)]
pub struct CurveData {
    pub is_curve_embedded: bool,
    pub embedded_animation_curve: Option<AnimationCurve>,
    pub animation_curve_id: Option<u32>,
}

impl Serialize for CurveData {
    fn deserialize<R>(reader: &mut Reader<R>) -> Result<Self>
            where R: Read + Seek {
        let mut chunk = Self {
            ..Default::default()
        };

        chunk.is_curve_embedded = reader.read_wide_bool()?;
        if chunk.is_curve_embedded {
            if reader.read_i32()? != -1 {
                warn!("Expected -1 on CurveData");
            }
            chunk.embedded_animation_curve = Some(AnimationCurve::deserialize(reader)?);
        } else {
            chunk.animation_curve_id = Some(reader.read_u32()?);
        }

        Ok(chunk)
    }

    fn serialize<W>(chunk: &Self, writer: &mut Writer<W>) -> Result<()>
            where W: Write + WriteBytesExt + Seek {
        writer.write_wide_bool(chunk.is_curve_embedded)?;
        if chunk.is_curve_embedded {
            writer.write_i32(-1)?;
            AnimationCurve::serialize(if let Some(eac) = chunk.embedded_animation_curve.as_ref() {
                eac
            } else {
                return Err(Error::new(ErrorKind::InvalidData, "Expected AnimationCurve but found None"));
            }, writer)?;
        } else {
            writer.write_u32(if let Some(aci) = chunk.animation_curve_id {
                aci
            } else {
                return Err(Error::new(ErrorKind::InvalidData, "Expected AnimationCurve pointer but found None"));
            })?;
        }

        Ok(())
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
    fn deserialize<R>(reader: &mut Reader<R>) -> Result<Self>
            where R: Read + Seek {
        let mut chunk = Self {
            ..Default::default()
        };

        chunk.text = reader.read_pointer_string()?;
        chunk.wrap = reader.read_wide_bool()?;
        chunk.alignment.magic_number = reader.read_i32()?;
        chunk.font_index = reader.read_i32()?;

        Ok(chunk)
    }
    
    fn serialize<W>(chunk: &Self, writer: &mut Writer<W>) -> Result<()>
            where W: Write + WriteBytesExt + Seek {
        writer.write_pointer_string(&chunk.text)?;
        writer.write_wide_bool(chunk.wrap)?;
        writer.write_i32(chunk.alignment.magic_number)?;
        writer.write_i32(chunk.font_index)?;

        Ok(())
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