use core::reader::Reader;
use std::{fs::File, io::BufReader};

pub mod core;

fn main() {
    tracing_subscriber::fmt().init();
    color_eyre::install().unwrap();

    let f = BufReader::new(File::open("data-uty.win").expect("Unable to open the data.win file"));
    let mut r = Reader::new(f, Some("data-uty.win".into()));
    r.deserialize_chunks().unwrap();
    r.version_info.set_version(2023, 4, 0, 0);
    r.deserialize().unwrap();
}
