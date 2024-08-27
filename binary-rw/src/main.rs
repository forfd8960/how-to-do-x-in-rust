use anyhow::Result;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::{
    fs::OpenOptions,
    io::{BufReader, BufWriter, Read, Write},
};

fn main() -> Result<()> {
    write_data_to_binary()?;
    read_data_from_binary()
}

fn write_data_to_binary() -> anyhow::Result<()> {
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("binary_data")?;

    let mut writer = BufWriter::new(file);

    let data = b"Hello, World!";

    println!("write: {} to file", String::from_utf8_lossy(data));
    writer.write_u32::<BigEndian>(data.len() as u32)?;
    writer.write_all(data)?;

    let msg = b"Good to see you, My Friend";

    println!("write: {} to file", String::from_utf8_lossy(msg));
    writer.write_u32::<BigEndian>(msg.len() as u32)?;
    writer.write_all(msg)?;

    writer.flush()?;

    println!("write binary data to file done");
    Ok(())
}

fn read_data_from_binary() -> Result<()> {
    let reader = OpenOptions::new().read(true).open("binary_data")?;
    let mut reader = BufReader::new(reader);

    let data_len = reader.read_u32::<BigEndian>()?;
    println!("data length: {}", data_len);

    let mut hello_word = vec![0; data_len as usize];
    reader.read_exact(&mut hello_word)?;
    println!("hello_word: {}", String::from_utf8_lossy(&hello_word));

    let msg_len = reader.read_u32::<BigEndian>()?;
    println!("msg length: {}", msg_len);

    let mut msg_data = vec![0; msg_len as usize];
    reader.read_exact(&mut msg_data)?;
    println!("msg: {}", String::from_utf8_lossy(&msg_data));

    Ok(())
}
