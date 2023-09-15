use serde::{Deserialize, Serialize};
use serde_bencode::{from_bytes, from_str, to_bytes, value::Value};
use serde_bytes::ByteBuf;
use sha1::{Digest, Sha1};
use std::env;
use std::io::Read;

#[derive(Debug, Serialize, Deserialize)]
struct Info {
    length: u64,
    name: String,
    #[serde(rename = "piece length")]
    piece_length: u64,
    pieces: ByteBuf,
}

#[derive(Debug, Serialize, Deserialize)]
struct MetaInfo {
    info: Info,
    announce: String,
}

struct HexSlice<'a>(&'a [u8]);

impl<'a> std::fmt::LowerHex for HexSlice<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "0x")?;
        }
        for &byte in self.0 {
            write!(f, "{:02x}", byte)?;
        }
        Ok(())
    }
}

fn decode(encoded_value: &str) -> Value {
    return from_str::<Value>(encoded_value).unwrap();
}

fn format(value: &Value) -> String {
    return match value {
        Value::Bytes(bytes) => format!("{:?}", std::str::from_utf8(bytes).unwrap()),
        Value::Int(i) => i.to_string(),
        Value::List(list) => format!(
            "[{}]",
            list.iter().map(format).collect::<Vec<String>>().join(",")
        ),

        Value::Dict(dict) => {
            let mut result = Vec::<String>::new();
            for (key, value) in dict {
                let key_str = String::from_utf8_lossy(key).to_string();
                result.push(format!("\"{}\":{}", key_str, format(value)));
            }
            result.sort();
            format!("{{{}}}", result.join(","))
        }
    };
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];
    if command == "decode" {
        let encoded_value = &args[2];

        let decoded_value = decode(encoded_value);

        println!("{}", format(&decoded_value));
    } else if command == "info" {
        let mut file = std::fs::File::open(&args[2]).unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();
        let decoded: MetaInfo = from_bytes(&buffer).unwrap();

        let hash = Sha1::digest(to_bytes(&decoded.info).unwrap());

        let pieces_hashes: Vec<_> = decoded
            .info
            .pieces
            .chunks(20)
            .map(|chunk| format!("{:x}", HexSlice(chunk)))
            .collect();

        println!("Tracker URL: {}", decoded.announce);
        println!("Length: {}", decoded.info.length);
        println!("Info Hash: {:x}", hash);
        println!("Piece Length: {}", decoded.info.piece_length);
        println!("Piece Hashes:\n{}", pieces_hashes.join("\n"));
    } else {
        println!("unknown command: {}", args[1])
    }
}
