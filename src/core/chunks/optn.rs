use std::{fmt::Write, io::{Read, Seek}};
use bitflags::bitflags;
use byteorder::WriteBytesExt;
use crate::core::{lists::GMSimpleList, reader::Reader, serializing::Serialize, writer::Writer, models::option::Constant};

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct OptionsFlags: u64 {
        const None = 0x0;
        const Fullscreen = 0x1;
        const InterpolatePixels = 0x2;
        const UseNewAudio = 0x4;
        const NoBorder = 0x8;
        const ShowCursor = 0x10;
        const Sizeable = 0x20;
        const StayOnTop = 0x40;
        const ChangeResolution = 0x80;
        const NoButtons = 0x100;
        const ScreenKey = 0x200;
        const HelpKey = 0x400;
        const QuitKey = 0x800;
        const SaveKey = 0x1000;
        const ScreenshotKey = 0x2000;
        const CloseSec = 0x4000;
        const Freeze = 0x8000;
        const ShowProgress = 0x10000;
        const LoadTransparent = 0x20000;
        const ScaleProgress = 0x40000;
        const DisplayErrors = 0x80000;
        const WriteErrors = 0x100000;
        const AbortErrors = 0x200000;
        const VariableErrors = 0x400000;
        const CreationEventOrder = 0x800000;
        const UseFrontTouch = 0x1000000;
        const UseRearTouch = 0x2000000;
        const UseFastCollision = 0x4000000;
        const FastCollisionCompatibility = 0x8000000;
        const DisableSandbox = 0x10000000;
        const CopyOnWriteEnabled = 0x20000000;
    }
}

#[derive(Clone)]
pub struct ChunkOPTN {
    pub unknown: u64,
    pub options: OptionsFlags,
    pub scale: i32,
    pub window_color: u32,
    pub color_depth: u32,
    pub resolution: u32,
    pub frequency: u32,
    pub vertex_sync: u32,
    pub priority: u32,
    pub splash_back_image: u32, // These are pointers of seemingly unused splash textures
    pub splash_front_image: u32,
    pub splash_load_image: u32,
    pub load_alpha: u32,
    pub constants: GMSimpleList<Constant>,
}

impl Default for ChunkOPTN {
    fn default() -> Self {
        Self {
            unknown: 0,
            options: OptionsFlags::None,
            scale: 0,
            window_color: 0,
            color_depth: 0,
            resolution: 0,
            frequency: 0,
            vertex_sync: 0,
            priority: 0,
            splash_back_image: 0,
            splash_front_image: 0,
            splash_load_image: 0,
            load_alpha: 0,
            constants: GMSimpleList::default(),
        }
    }
}

impl Serialize for ChunkOPTN {
    fn deserialize<R>(reader: &mut Reader<R>) -> Self
        where R: Read + Seek,
    {
        let mut chunk = Self {
            ..Default::default()
        };

        reader.version_info.option_bit_flag = reader.read_i32().expect("Failed to read option bit flag") == i32::MIN;
        reader.seek_relative(-4).expect("Failed to seek back");

        if reader.version_info.option_bit_flag {
            chunk.unknown = reader.read_u64().expect("Failed to read unknown");
            chunk.options = OptionsFlags::from_bits_truncate(
                reader.read_u64().expect("Failed to read options"),
            );
            chunk.scale = reader.read_i32().expect("Failed to read scale");
            chunk.window_color = reader.read_u32().expect("Failed to read window_color");
            chunk.color_depth = reader.read_u32().expect("Failed to read color_depth");
            chunk.resolution = reader.read_u32().expect("Failed to read resolution");
            chunk.frequency = reader.read_u32().expect("Failed to read frequency");
            chunk.vertex_sync = reader.read_u32().expect("Failed to read vertex_sync");
            chunk.priority = reader.read_u32().expect("Failed to read priority");
            chunk.splash_back_image = reader.read_u32().expect("Failed to read splash_back_image");
            chunk.splash_front_image = reader.read_u32().expect("Failed to read splash_front_image");
            chunk.splash_load_image = reader.read_u32().expect("Failed to read splash_load_image");
            chunk.load_alpha = reader.read_u32().expect("Failed to read load_alpha");
        } else {
            let mut options = 0;
            let mut read_option = |reader: &mut Reader<R>, option: OptionsFlags| {
                if reader.read_wide_bool().expect("Failed to read option") {
                    options |= option.bits();
                }
            };
            read_option(reader, OptionsFlags::Fullscreen);
            read_option(reader, OptionsFlags::InterpolatePixels);
            read_option(reader, OptionsFlags::UseNewAudio);
            read_option(reader, OptionsFlags::NoBorder);
            read_option(reader, OptionsFlags::ShowCursor);
            chunk.scale = reader.read_i32().expect("Failed to read scale");
            read_option(reader, OptionsFlags::Sizeable);
            read_option(reader, OptionsFlags::StayOnTop);
            chunk.window_color = reader.read_u32().expect("Failed to read window_color");
            read_option(reader, OptionsFlags::ChangeResolution);
            chunk.color_depth = reader.read_u32().expect("Failed to read color_depth");
            chunk.resolution = reader.read_u32().expect("Failed to read resolution");
            chunk.frequency = reader.read_u32().expect("Failed to read frequency");
            read_option(reader, OptionsFlags::NoButtons);
            chunk.vertex_sync = reader.read_u32().expect("Failed to read vertex_sync");
            read_option(reader, OptionsFlags::ScreenKey);
            read_option(reader, OptionsFlags::HelpKey);
            read_option(reader, OptionsFlags::QuitKey);
            read_option(reader, OptionsFlags::SaveKey);
            read_option(reader, OptionsFlags::ScreenshotKey);
            read_option(reader, OptionsFlags::CloseSec);
            chunk.priority = reader.read_u32().expect("Failed to read priority");
            read_option(reader, OptionsFlags::Freeze);
            read_option(reader, OptionsFlags::ShowProgress);
            chunk.splash_back_image = reader.read_u32().expect("Failed to read splash_back_image");
            chunk.splash_front_image = reader.read_u32().expect("Failed to read splash_front_image");
            chunk.splash_load_image = reader.read_u32().expect("Failed to read splash_load_image");
            read_option(reader, OptionsFlags::LoadTransparent);
            chunk.load_alpha = reader.read_u32().expect("Failed to read load_alpha");
            read_option(reader, OptionsFlags::ScaleProgress);
            read_option(reader, OptionsFlags::DisplayErrors);
            read_option(reader, OptionsFlags::WriteErrors);
            read_option(reader, OptionsFlags::AbortErrors);
            read_option(reader, OptionsFlags::VariableErrors);
            read_option(reader, OptionsFlags::CreationEventOrder);
            chunk.options = OptionsFlags::from_bits_truncate(options);
        }

        chunk.constants.deserialize(reader, None, None);

        chunk
    }

    fn serialize<W>(chunk: &Self, writer: &mut Writer<W>)
        where W: Write + WriteBytesExt + Seek,
    {
        if writer.version_info.option_bit_flag {
            writer.write_u64(chunk.unknown).expect("Failed to write unknown");
            writer.write_u64(chunk.options.bits()).expect("Failed to write options");
            writer.write_i32(chunk.scale).expect("Failed to write scale");
            writer.write_u32(chunk.window_color).expect("Failed to write window_color");
            writer.write_u32(chunk.color_depth).expect("Failed to write color_depth");
            writer.write_u32(chunk.resolution).expect("Failed to write resolution");
            writer.write_u32(chunk.frequency).expect("Failed to write frequency");
            writer.write_u32(chunk.vertex_sync).expect("Failed to write vertex_sync");
            writer.write_u32(chunk.priority).expect("Failed to write priority");
            writer.write_u32(chunk.splash_back_image).expect("Failed to write splash_back_image");
            writer.write_u32(chunk.splash_front_image).expect("Failed to write splash_front_image");
            writer.write_u32(chunk.splash_load_image).expect("Failed to write splash_load_image");
            writer.write_u32(chunk.load_alpha).expect("Failed to write load_alpha");
        } else {
            let write_option = |writer: &mut Writer<W>, option: OptionsFlags| {
                writer.write_wide_bool((chunk.options & option) == option).expect("Failed to write option");
            };
            write_option(writer, OptionsFlags::Fullscreen);
            write_option(writer, OptionsFlags::InterpolatePixels);
            write_option(writer, OptionsFlags::UseNewAudio);
            write_option(writer, OptionsFlags::NoBorder);
            write_option(writer, OptionsFlags::ShowCursor);
            writer.write_i32(chunk.scale).expect("Failed to write scale");
            write_option(writer, OptionsFlags::Sizeable);
            write_option(writer, OptionsFlags::StayOnTop);
            writer.write_u32(chunk.window_color).expect("Failed to write window_color");
            write_option(writer, OptionsFlags::ChangeResolution);
            writer.write_u32(chunk.color_depth).expect("Failed to write color_depth");
            writer.write_u32(chunk.resolution).expect("Failed to write resolution");
            writer.write_u32(chunk.frequency).expect("Failed to write frequency");
            write_option(writer, OptionsFlags::NoButtons);
            writer.write_u32(chunk.vertex_sync).expect("Failed to write vertex_sync");
            write_option(writer, OptionsFlags::ScreenKey);
            write_option(writer, OptionsFlags::HelpKey);
            write_option(writer, OptionsFlags::QuitKey);
            write_option(writer, OptionsFlags::SaveKey);
            write_option(writer, OptionsFlags::ScreenshotKey);
            write_option(writer, OptionsFlags::CloseSec);
            writer.write_u32(chunk.priority).expect("Failed to write priority");
            write_option(writer, OptionsFlags::Freeze);
            write_option(writer, OptionsFlags::ShowProgress);
            writer.write_u32(chunk.load_alpha).expect("Failed to write load_alpha");
            write_option(writer, OptionsFlags::ScaleProgress);
            write_option(writer, OptionsFlags::DisplayErrors);
            write_option(writer, OptionsFlags::WriteErrors);
            write_option(writer, OptionsFlags::AbortErrors);
            write_option(writer, OptionsFlags::VariableErrors);
            write_option(writer, OptionsFlags::CreationEventOrder);
        }
        
        chunk.constants.serialize(writer, None, None);
    }
}
