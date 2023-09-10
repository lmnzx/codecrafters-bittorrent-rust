use serde::Deserialize;
use serde_bencode::{self, de, value::Value};
use serde_bytes::ByteBuf;
use std::env;

#[derive(Debug, Deserialize)]
struct Node(String, i64);

#[derive(Debug, Deserialize)]
struct File {
    path: Vec<String>,
    length: i64,
    #[serde(default)]
    md5sum: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Info {
    name: String,
    pieces: ByteBuf,
    #[serde(rename = "piece length")]
    piece_length: i64,
    #[serde(default)]
    md5sum: Option<String>,
    #[serde(default)]
    length: Option<i64>,
    #[serde(default)]
    files: Option<Vec<File>>,
    #[serde(default)]
    private: Option<u8>,
    #[serde(default)]
    path: Option<Vec<String>>,
    #[serde(default)]
    #[serde(rename = "root hash")]
    root_hash: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Torrent {
    info: Info,
    #[serde(default)]
    announce: Option<String>,
    #[serde(default)]
    nodes: Option<Vec<Node>>,
    #[serde(default)]
    encoding: Option<String>,
    #[serde(default)]
    httpseeds: Option<Vec<String>>,
    #[serde(default)]
    #[serde(rename = "announce-list")]
    announce_list: Option<Vec<Vec<String>>>,
    #[serde(default)]
    #[serde(rename = "creation date")]
    creation_date: Option<i64>,
    #[serde(rename = "comment")]
    comment: Option<String>,
    #[serde(default)]
    #[serde(rename = "created by")]
    created_by: Option<String>,
}

fn decode(encoded_value: &str) -> Value {
    return serde_bencode::from_str::<Value>(encoded_value).unwrap();
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
        let torrent_file = std::fs::read(&args[2]).unwrap();
        let decoded_value = de::from_bytes::<Torrent>(&torrent_file).unwrap();
        println!("Length: {}", decoded_value.info.length.unwrap_or_default());
    } else {
        println!("unknown command: {}", args[1])
    }
}
