use bstr::BString;

pub mod lists;
pub mod reader;
pub mod writer;
pub mod chunks;
pub mod models;
pub mod serializing;
pub mod string;

#[derive(Clone, Debug)]
pub struct GMVersionInfo {
    pub major: i32, // The version of the IDE the data file was built with
    pub minor: i32,
    pub release: i32,
    pub build: i32,
    pub format_id: i8, // Bytecode format
    pub align_chunks_to_16: bool, // Only from version 2.2.2>
    pub align_strings_to_4: bool,
    pub align_backgrounds_to_8: bool,
    pub room_object_pre_create: bool, // If the data file uses Pre-Create events for rooms and objects
    pub different_var_counts: bool, // If some unknown variables in the VARI chunk have different values (needs format_id >= 14)
    pub option_bit_flag: bool, // If the data file uses option flags in the OPTN chunk
    pub run_from_ide: bool, // If the data file was run from the IDE
    pub short_circuit: bool, // If the VM bytecode short-circuits logical and/or operations
    pub builtin_audio_group_id: i32, // The ID of the first audio group in the data file
}

impl Default for GMVersionInfo {
    fn default() -> Self {
        Self {
            major: 1,
            minor: 0,
            release: 0,
            build: 0,
            format_id: 0,
            align_chunks_to_16: true,
            align_strings_to_4: true,
            align_backgrounds_to_8: true,
            room_object_pre_create: false,
            different_var_counts: false,
            option_bit_flag: true,
            run_from_ide: false,
            short_circuit: true,
            builtin_audio_group_id: 0,
        }
    }
}

impl GMVersionInfo {
    fn evaluate_builtin_audio_group_id(&mut self) {
        self.builtin_audio_group_id = (!(self.major >= 2 || (self.major == 1 && (self.build >= 1354 || (self.build >= 161 && self.build < 1000))))) as _;
    }

    pub fn set_version(&mut self, major: i32, minor: i32, release: i32, build: i32) {
        (|| {
            if self.major < major {
                self.major = major;
                self.minor = minor;
                self.release = release;
                self.build = build;
                return;
            }
            if self.major > major {
                return;
            }
            if self.minor < minor {
                self.minor = minor;
                self.release = release;
                self.build = build;
                return;
            }
            if self.minor > minor {
                return;
            }
            if self.release < release {
                self.release = release;
                self.build = build;
                return;
            }
            if self.release > release {
                return;
            }
            if self.build < build {
                self.build = build;
            }
        })();
        self.evaluate_builtin_audio_group_id();
    }

    pub fn is_version_at_least(&self, major: i32, minor: i32, release: i32, build: i32) -> bool {
        if self.major != major {
            return self.major > major;
        }
        if self.minor != minor {
            return self.minor > minor;
        }
        if self.release != release {
            return self.release > release;
        }
        if self.build != build {
            return self.build > build;
        }
        true
    }
}

#[derive(Default, Clone)]
pub struct Chunk {
    pub name: BString,
    pub length: u64,
    pub start_offset: u64,
    pub end_offset: u64,
}

#[derive(Default, Clone)]
pub struct GlobalData {
    lang_entry_count: i32,
}