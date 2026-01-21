use base64::{
    Engine as _, alphabet,
    engine::{self, general_purpose},
};
use bincode::{Decode, Encode, config};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Config {
    name: String,
    enabled: bool,
    timeout_ms: Option<u64>,
}

#[derive(Encode, Decode, Debug, PartialEq)]
struct Person {
    name: String,
    age: u32,
    email: String,
}

fn main() -> anyhow::Result<()> {
    to_json()?;
    to_yaml()?;

    to_toml()?;

    work_with_data_hex()?;

    work_with_data_base64()?;

    encode_decode_bincode()?;
    Ok(())
}

fn to_json() -> anyhow::Result<()> {
    let cfg = Config {
        name: "demo".into(),
        enabled: true,
        timeout_ms: None,
    };
    let json = serde_json::to_string(&cfg)?; // compact
    let pretty = serde_json::to_string_pretty(&cfg)?; // pretty-printed

    println!("JSON output:\n{}", json);
    println!();

    println!("Pretty JSON output:\n{}", pretty);
    println!();

    let from: Config = serde_json::from_str(&json)?;
    println!("Deserialized from JSON: {:?}", from);
    println!();
    Ok(())
}

fn to_yaml() -> anyhow::Result<()> {
    let cfg = Config {
        name: "demo".into(),
        enabled: true,
        timeout_ms: Some(300),
    };
    let yaml = serde_yaml::to_string(&cfg)?;
    println!("YAML output:\n{}", yaml);

    let from: Config = serde_yaml::from_str(&yaml)?;
    println!("Deserialized from YAML: {:?}", from);
    Ok(())
}

fn to_toml() -> anyhow::Result<()> {
    let cfg = Config {
        name: "demo".into(),
        enabled: true,
        timeout_ms: Some(300),
    };

    println!();

    let toml = toml::to_string(&cfg)?;
    println!("TOML output:\n{}", toml);

    let from: Config = toml::from_str(&toml)?;
    println!("Deserialized from TOML: {:?}", from);
    Ok(())
}

fn encode_decode_bincode() -> anyhow::Result<()> {
    let person = Person {
        name: "Alice".to_string(),
        age: 30,
        email: "alice@example.com".to_string(),
    };

    println!();

    // Encode to Vec<u8>
    let encoded = bincode::encode_to_vec(&person, config::standard())?;
    println!("Encoded bytes: {:?}", encoded);
    println!("Size: {} bytes", encoded.len());

    // Decode from bytes
    let (decoded, _): (Person, usize) = bincode::decode_from_slice(&encoded, config::standard())?;
    println!("Decoded: {:?}", decoded);

    assert_eq!(person, decoded);
    println!();
    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    desc: String,
    #[serde(serialize_with = "serialize_hex", deserialize_with = "deserialize_hex")]
    bytes: Vec<u8>,
}

fn serialize_hex<S>(bytes: &Vec<u8>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let s = hex::encode(bytes);
    serializer.serialize_str(&s)
}

fn deserialize_hex<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    hex::decode(&s).map_err(serde::de::Error::custom)
}

fn work_with_data_hex() -> anyhow::Result<()> {
    let data = Data {
        desc: "Example data".into(),
        bytes: vec![0xde, 0xad, 0xbe, 0xef],
    };

    let encode = serde_json::to_string(&data)?;
    println!("Serialized json: {}", encode);

    let decoded: Data = serde_json::from_str(&encode)?;
    println!("Deserialized from json: {:?}", decoded);

    Ok(())
}

fn serialize_data_base64<S>(bytes: &Vec<u8>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let s = general_purpose::STANDARD.encode(bytes);
    serializer.serialize_str(&s)
}

fn deserialize_data_base64<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;

    general_purpose::STANDARD
        .decode(&s)
        .map_err(serde::de::Error::custom)
}

#[derive(Serialize, Deserialize, Debug)]
struct DataBase64 {
    #[serde(
        serialize_with = "serialize_data_base64",
        deserialize_with = "deserialize_data_base64"
    )]
    bytes: Vec<u8>,
}

fn work_with_data_base64() -> anyhow::Result<()> {
    println!("work with base64 encoded data:");
    let data = DataBase64 {
        bytes: vec![0xde, 0xad, 0xbe, 0xef],
    };

    let encode = serde_json::to_string(&data)?;
    println!("Serialized json: {}", encode);

    let decoded: DataBase64 = serde_json::from_str(&encode)?;
    println!("Deserialized from json: {:?}", decoded);

    Ok(())
}
