use std::{io::{Seek, Read}, fmt::Write};
use bitflags::bitflags;
use bstr::BString;
use byteorder::WriteBytesExt;
use crate::core::{reader::Reader, serializing::Serialize, writer::Writer};

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct InfoFlags: u32 {
        const None = 0x0000; // No flags
        const Fullscreen = 0x0001; // Start fullscreen
        const SyncVertex1 = 0x0002; // Use synchronization to avoid tearing
        const SyncVertex2 = 0x0004;
        const Interpolate = 0x0008; // Interpolate colors between pixels
        const Scale = 0x0010; // Scaling: Keep aspect ratio
        const ShowCursor = 0x0020; // Display cursor
        const Sizeable = 0x0040; // Allow window resize
        const ScreenKey = 0x0080; // Allow fullscreen switching
        const SyncVertex3 = 0x0100;
        const StudioVersionB1 = 0x0200;
        const StudioVersionB2 = 0x0400;
        const StudioVersionB3 = 0x0800;
        const StudioVersionMask = 0x0e00; // studio_version = (info_flags & InfoFlags::StudioVersionMask) >> 9
        const SteamOrPlayer = 0x1000; // Steam or YoYo Player
        const LocalDataEnabled = 0x2000;
        const BorderlessWindow = 0x4000; // Borderless Window
        const DefaultCodeKind = 0x8000;
        const LicenseExclusions = 0x10000;
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct FunctionClassification: u64 {
        const None = 0x0;
        const Internet = 0x1;
        const Joystick = 0x2;
        const Gamepad = 0x4;
        const ReadScreenPixels = 0x10;
        const Math = 0x20;
        const Action = 0x40;
        const D3dState = 0x80;
        const D3dPrimitive = 0x100;
        const DataStructure = 0x200;
        const FileLegacy = 0x400;
        const Ini = 0x800;
        const Filename = 0x1000;
        const Directory = 0x2000;
        const Shell = 0x4000;
        const Obsolete = 0x8000;
        const Http = 0x10000;
        const JsonZip = 0x20000;
        const Debug = 0x40000;
        const Motion = 0x80000;
        const Collision = 0x100000;
        const Instance = 0x200000;
        const Room = 0x400000;
        const Game = 0x800000;
        const Display = 0x1000000;
        const Device = 0x2000000;
        const Window = 0x4000000;
        const Draw = 0x8000000;
        const Texture = 0x10000000;
        const Graphics = 0x20000000;
        const String = 0x40000000;
        const Tile = 0x80000000;
        const Surface = 0x100000000;
        const Skeleton = 0x200000000;
        const Io = 0x400000000;
        const GmSystem = 0x800000000;
        const Array = 0x1000000000;
        const External = 0x2000000000;
        const Push = 0x4000000000;
        const Date = 0x8000000000;
        const Particle = 0x10000000000;
        const Resource = 0x20000000000;
        const Html5 = 0x40000000000;
        const Sound = 0x80000000000;
        const Audio = 0x100000000000;
        const Event = 0x200000000000;
        const Script = 0x400000000000;
        const Text = 0x800000000000;
        const Analytics = 0x1000000000000;
        const Object = 0x2000000000000;
        const Asset = 0x4000000000000;
        const Achievement = 0x8000000000000;
        const Cloud = 0x10000000000000;
        const Ads = 0x20000000000000;
        const Os = 0x40000000000000;
        const Iap = 0x80000000000000;
        const Facebook = 0x100000000000000;
        const Physics = 0x200000000000000;
        const Swf = 0x400000000000000;
        const PlatformSpecific = 0x800000000000000;
        const Buffer = 0x1000000000000000;
        const Steam = 0x2000000000000000;
        const SteamUgc = 0x2010000000000000;
        const Shader = 0x4000000000000000;
        const Vertex = 0x8000000000000000;
    }
}

#[derive(Clone)]
pub struct ChunkGEN8 {
    pub disable_debug: bool,
    pub format_id: i8,
    pub unknown: i16,
    pub filename: BString,
    pub config: BString,
    pub last_object_id: i32,
    pub last_tile_id: i32,
    pub game_id: i32,
    pub legacy_guid: [u8; 16],
    pub game_name: BString,
    pub major: i32,
    pub minor: i32,
    pub release: i32,
    pub build: i32,
    pub default_window_width: i32,
    pub default_window_height: i32,
    pub info: InfoFlags,
    pub license_md5: [u8; 16],
    pub license_crc32: i32,
    pub timestamp: i64,
    pub display_name: BString,
    pub active_targets: i64,
    pub function_classifications: FunctionClassification,
    pub steam_app_id: i32,
    pub debugger_port: i32,
    pub room_order: Vec<i32>,
    pub gms2_random_uid: Vec<i64>,
    pub gms2_fps: f32,
    pub gms2_allow_statistics: bool,
    pub gms2_game_guid: Vec<u8>,
}

impl Default for ChunkGEN8 {
    fn default() -> Self {
        Self {
            disable_debug: true,
            format_id: 0,
            unknown: 0,
            filename: BString::default(),
            config: BString::default(),
            last_object_id: 0,
            last_tile_id: 0,
            game_id: 0,
            legacy_guid: [0; 16],
            game_name: BString::default(),
            major: 0,
            minor: 0,
            release: 0,
            build: 0,
            default_window_width: 0,
            default_window_height: 0,
            info: InfoFlags::None,
            license_md5: [0; 16],
            license_crc32: 0,
            timestamp: 0,
            display_name: BString::default(),
            active_targets: 0,
            function_classifications: FunctionClassification::None,
            steam_app_id: 0,
            debugger_port: 0,
            room_order: Vec::new(),
            gms2_random_uid: Vec::new(),
            gms2_fps: 0.0,
            gms2_allow_statistics: false,
            gms2_game_guid: Vec::new(),
        }
    }
}

impl Serialize for ChunkGEN8 {
    fn deserialize<R>(reader: &mut Reader<R>) -> Self
        where R: Read + Seek,
    {
        let mut chunk = Self {
            ..Default::default()
        };

        chunk.disable_debug = reader.read_bool().expect("Failed to read disable_debug");
        chunk.format_id = reader.read_i8().expect("Failed to read format_id");
        reader.version_info.format_id = chunk.format_id;
        chunk.unknown = reader.read_i16().expect("Failed to read unknown");
        chunk.filename = reader.read_pointer_string().expect("Failed to read filename");
        chunk.config = reader.read_pointer_string().expect("Failed to read config");
        chunk.last_object_id = reader.read_i32().expect("Failed to read last_object_id");
        chunk.last_tile_id = reader.read_i32().expect("Failed to read last_tile_id");
        chunk.game_id = reader.read_i32().expect("Failed to read game_id");
        chunk.legacy_guid = reader.read_bytes::<16>().expect("Failed to read legacy_guid");
        chunk.game_name = reader.read_pointer_string().expect("Failed to read game_name");
        chunk.major = reader.read_i32().expect("Failed to read major");
        chunk.minor = reader.read_i32().expect("Failed to read minor");
        chunk.release = reader.read_i32().expect("Failed to read release");
        chunk.build = reader.read_i32().expect("Failed to read build");
        reader.version_info.set_version(chunk.major, chunk.minor, chunk.release, chunk.build);
        chunk.default_window_width = reader.read_i32().expect("Failed to read default_window_width");
        chunk.default_window_height = reader.read_i32().expect("Failed to read default_window_height");
        chunk.info = InfoFlags::from_bits_retain(reader.read_u32().expect("Failed to read info"));
        chunk.license_crc32 = reader.read_i32().expect("Failed to read license_crc32");
        chunk.license_md5 = reader.read_bytes::<16>().expect("Failed to read license_md5");
        chunk.timestamp = reader.read_i64().expect("Failed to read timestamp");
        chunk.display_name = reader.read_pointer_string().expect("Failed to read display_name");
        chunk.active_targets = reader.read_i64().expect("Failed to read active_targets");
        chunk.function_classifications = FunctionClassification::from_bits_retain(reader.read_u64().expect("Failed to read function_classifications"));
        chunk.steam_app_id = reader.read_i32().expect("Failed to read steam_app_id");
        if chunk.format_id >= 14 {
            chunk.debugger_port = reader.read_i32().expect("Failed to read debugger_port");
        }
        for _ in 0..reader.read_i32().expect("Failed to read room_order length") {
            chunk.room_order.push(reader.read_i32().expect("Failed to read room_order value"));
        }
        if reader.version_info.major >= 2 {
            for _ in 0..5 {
                chunk.gms2_random_uid.push(reader.read_i64().expect("Failed to read gms2_random_uid value"));
            }
            chunk.gms2_fps = reader.read_f32().expect("Failed to read gms2_fps");
            chunk.gms2_allow_statistics = reader.read_wide_bool().expect("Failed to read gms2_allow_statistics");
            chunk.gms2_game_guid = reader.read_bytes::<16>().expect("Failed to read gms2_game_guid").into();
        }

        chunk
    }

    fn serialize<W>(chunk: &Self, writer: &mut Writer<W>)
            where W: Write + WriteBytesExt + Seek,
    {
        writer.write_bool(chunk.disable_debug).expect("Failed to write disable_debug");
        writer.write_i8(chunk.format_id).expect("Failed to write format_id");
        writer.write_i16(chunk.unknown).expect("Failed to write unknown");
        writer.write_pointer_string(&chunk.filename).expect("Failed to write filename");
        writer.write_pointer_string(&chunk.config).expect("Failed to write config");
        writer.write_i32(chunk.last_object_id).expect("Failed to write last_object_id");
        writer.write_i32(chunk.last_tile_id).expect("Failed to write last_tile_id");
        writer.write_i32(chunk.game_id).expect("Failed to write game_id");
        writer.write_bytes(&chunk.legacy_guid).expect("Failed to write legacy_guid");
        writer.write_pointer_string(&chunk.game_name).expect("Failed to write game_name");
        writer.write_i32(chunk.major).expect("Failed to write major");
        writer.write_i32(chunk.minor).expect("Failed to write minor");
        writer.write_i32(chunk.release).expect("Failed to write release");
        writer.write_i32(chunk.build).expect("Failed to write build");
        writer.write_i32(chunk.default_window_width).expect("Failed to write default_window_width");
        writer.write_i32(chunk.default_window_height).expect("Failed to write default_window_height");
        writer.write_u32(chunk.info.bits()).expect("Failed to write info");
        writer.write_i32(chunk.license_crc32).expect("Failed to write license_crc32");
        writer.write_bytes(&chunk.license_md5).expect("Failed to write license_md5");
        writer.write_i64(chunk.timestamp).expect("Failed to write timestamp");
        writer.write_pointer_string(&chunk.display_name).expect("Failed to write display_name");
        writer.write_i64(chunk.active_targets).expect("Failed to write active_targets");
        writer.write_u64(chunk.function_classifications.bits()).expect("Failed to write function_classifications");
        writer.write_i32(chunk.steam_app_id).expect("Failed to write steam_app_id");
        if chunk.format_id >= 14 {
            writer.write_i32(chunk.debugger_port).expect("Failed to write debugger_port");
        }
        writer.write_i32(chunk.room_order.len() as i32).expect("Failed to write room_order length");
        for room in &chunk.room_order {
            writer.write_i32(*room).expect("Failed to write room_order value");
        }
        if writer.version_info.major >= 2 {
            for uid in &chunk.gms2_random_uid {
                writer.write_i64(*uid).expect("Failed to write gms2_random_uid value");
            }
            writer.write_f32(chunk.gms2_fps).expect("Failed to write gms2_fps");
            writer.write_wide_bool(chunk.gms2_allow_statistics).expect("Failed to write gms2_allow_statistics");
            writer.write_bytes(&chunk.gms2_game_guid).expect("Failed to write gms2_game_guid");
        }
    }
}