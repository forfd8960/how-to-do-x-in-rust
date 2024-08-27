use std::{fs::OpenOptions, io::Write};

use flate2::{write::ZlibEncoder, Compression};

fn main() {
    println!("writing bytes and compress with zlib");
    let bytes = b"hello world";
    write_bytes_to_file(bytes, "hello_word").unwrap();
}

pub fn write_bytes_to_file(bytes: &[u8], filename: &str) -> anyhow::Result<()> {
    println!("writing {:?} to file", bytes);

    let blob = OpenOptions::new()
        .append(true)
        .create(true)
        .open(filename)?;
    println!("created file");

    let mut e = ZlibEncoder::new(blob, Compression::default());
    e.write_all(&bytes)?;
    e.finish()?;

    Ok(())
}
