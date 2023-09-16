// use serde::{Deserialize, Serialize};
// use serde_bencode::{from_bytes, from_str, to_bytes, value::Value};
// use serde_bytes::ByteBuf;
// use sha1::{Digest, Sha1};
// use std::env;
// use std::fmt::Display;
// use std::io::{Read, Write};

// #[derive(Debug, Serialize, Deserialize)]
// struct Info {
//     length: u64,
//     name: String,
//     #[serde(rename = "piece length")]
//     piece_length: u64,
//     pieces: ByteBuf,
// }

// #[derive(Debug, Serialize, Deserialize)]
// struct MetaInfo {
//     info: Info,
//     announce: String,
// }

// struct HexSlice<'a>(&'a [u8]);

// impl<'a> std::fmt::LowerHex for HexSlice<'a> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         if f.alternate() {
//             write!(f, "0x")?;
//         }
//         for &byte in self.0 {
//             write!(f, "{:02x}", byte)?;
//         }
//         Ok(())
//     }
// }

// pub fn vec_to_hex(bytes: &[u8]) -> String {
//     bytes.iter().map(|byte| format!("{:02x}", byte)).collect()
// }

// #[derive(Serialize, Deserialize, Debug)]
// struct TrackerResponse {
//     interval: i64,
//     peers: ByteBuf,
// }
// impl Display for TrackerResponse {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", self.get_peers().join("\n"))
//     }
// }
// impl TrackerResponse {
//     fn get_peers(&self) -> Vec<String> {
//         self.peers
//             .chunks(6)
//             .map(|chunk| {
//                 let ip = chunk[0..4]
//                     .iter()
//                     .map(|b| b.to_string())
//                     .collect::<Vec<String>>()
//                     .join(".");
//                 let port = u16::from_be_bytes([chunk[4], chunk[5]]);
//                 format!("{}:{}", ip, port)
//             })
//             .collect()
//     }
// }

// fn decode(encoded_value: &str) -> Value {
//     return from_str::<Value>(encoded_value).unwrap();
// }

// fn format(value: &Value) -> String {
//     return match value {
//         Value::Bytes(bytes) => format!("{:?}", std::str::from_utf8(bytes).unwrap()),
//         Value::Int(i) => i.to_string(),
//         Value::List(list) => format!(
//             "[{}]",
//             list.iter().map(format).collect::<Vec<String>>().join(",")
//         ),

//         Value::Dict(dict) => {
//             let mut result = Vec::<String>::new();
//             for (key, value) in dict {
//                 let key_str = String::from_utf8_lossy(key).to_string();
//                 result.push(format!("\"{}\":{}", key_str, format(value)));
//             }
//             result.sort();
//             format!("{{{}}}", result.join(","))
//         }
//     };
// }
// fn main() {
//     let args: Vec<String> = env::args().collect();
//     let command = &args[1];
//     if command == "decode" {
//         let encoded_value = &args[2];

//         let decoded_value = decode(encoded_value);

//         println!("{}", format(&decoded_value));
//     } else if command == "info" {
//         let mut file = std::fs::File::open(&args[2]).unwrap();
//         let mut buffer = Vec::new();
//         file.read_to_end(&mut buffer).unwrap();
//         let decoded: MetaInfo = from_bytes(&buffer).unwrap();

//         let hash = Sha1::digest(to_bytes(&decoded.info).unwrap());

//         let pieces_hashes: Vec<_> = decoded
//             .info
//             .pieces
//             .chunks(20)
//             .map(|chunk| format!("{:x}", HexSlice(chunk)))
//             .collect();

//         println!("Tracker URL: {}", decoded.announce);
//         println!("Length: {}", decoded.info.length);
//         println!("Info Hash: {:x}", hash);
//         println!("Piece Length: {}", decoded.info.piece_length);
//         println!("Piece Hashes:\n{}", pieces_hashes.join("\n"));
//     } else if command == "peers" {
//         let mut file = std::fs::File::open(&args[2]).unwrap();
//         let mut buffer = Vec::new();
//         file.read_to_end(&mut buffer).unwrap();
//         let decoded: MetaInfo = from_bytes(&buffer).unwrap();

//         let hash = format!("{:x}", Sha1::digest(to_bytes(&decoded.info).unwrap()));

//         let hash =
//             hash.chars()
//                 .collect::<Vec<char>>()
//                 .chunks(2)
//                 .fold(String::new(), |acc, chuck| {
//                     let symbol = chuck.iter().collect::<String>();
//                     let c = u8::from_str_radix(&symbol, 16).unwrap();
//                     match c {
//                         45 | 46 | 48..=57 | 65..=90 | 95 | 97..=122 => {
//                             acc + &char::from(c).to_string()
//                         }
//                         _ => acc + "%" + &symbol,
//                     }
//                 });

//         let peer_id = "00112233445566778899";
//         let port = 6881;
//         let uploaded = 0;
//         let downloaded = 0;
//         let left = decoded.info.length;
//         let compact = 1;

//         let url = format!(
//             "{}?info_hash={}&peer_id={}&port={}&uploaded={}&downloaded={}&left={}&compact={}",
//             decoded.announce, hash, peer_id, port, uploaded, downloaded, left, compact
//         );

//         let response = reqwest::blocking::get(&url).unwrap().bytes().unwrap();
//         let decoded: TrackerResponse = from_bytes(&response).unwrap();
//         println!("{}", decoded);
//     } else if command == "handshake" {
//         let mut file = std::fs::File::open(&args[2]).unwrap();
//         let mut buffer = Vec::new();
//         file.read_to_end(&mut buffer).unwrap();
//         let decoded: MetaInfo = from_bytes(&buffer).unwrap();

//         let hash = Sha1::digest(to_bytes(&decoded.info).unwrap());

//         let peer_id = "00112233445566778899";

//         let ip = &args[3];

//         // length of protocol string
//         let mut message: Vec<u8> = vec![19];
//         // protocol string
//         message.extend(b"BitTorrent protocol");
//         // reserved bytes
//         message.extend(vec![0; 8]);
//         // sha1 infohash (20 bytes)
//         message.extend(hash);
//         // peer id (20 bytes)
//         message.extend(peer_id.as_bytes());

//         let mut stream = std::net::TcpStream::connect(ip).unwrap();

//         let _ = stream.write(&message);
//         let mut response = vec![0; message.len()];
//         let _ = stream.read(&mut response);
//         let response_peer_id = &response[response.len() - 20..];
//         println!("Peer ID: {}", vec_to_hex(response_peer_id));
//     } else if command == "download_piece" {
//         assert_eq!("-o", &args[2]);

//         let output_file = &args[3];

//         let mut file = std::fs::File::open(&args[4]).unwrap();

//         let piece_index = args[5].parse::<usize>().unwrap();

//         let mut buffer = Vec::new();
//         file.read_to_end(&mut buffer).unwrap();
//         let decoded: MetaInfo = from_bytes(&buffer).unwrap();

//         let hash = format!("{:x}", Sha1::digest(to_bytes(&decoded.info).unwrap()));

//         let hash =
//             hash.chars()
//                 .collect::<Vec<char>>()
//                 .chunks(2)
//                 .fold(String::new(), |acc, chuck| {
//                     let symbol = chuck.iter().collect::<String>();
//                     let c = u8::from_str_radix(&symbol, 16).unwrap();
//                     match c {
//                         45 | 46 | 48..=57 | 65..=90 | 95 | 97..=122 => {
//                             acc + &char::from(c).to_string()
//                         }
//                         _ => acc + "%" + &symbol,
//                     }
//                 });

//         let peer_id = "00112233445566778899";
//         let port = 6881;
//         let uploaded = 0;
//         let downloaded = 0;
//         let left = decoded.info.length;
//         let compact = 1;

//         let url = format!(
//             "{}?info_hash={}&peer_id={}&port={}&uploaded={}&downloaded={}&left={}&compact={}",
//             decoded.announce, hash, peer_id, port, uploaded, downloaded, left, compact
//         );

//         let response = reqwest::blocking::get(&url).unwrap().bytes().unwrap();
//         let peers: TrackerResponse = from_bytes(&response).unwrap();
//         let peerslist = format!("{}", peers);
//         let peerslist = peerslist.split("\n").collect::<Vec<&str>>();

//         println!("Connecting to {}", peerslist[0]);

//         // let peers = torrent_file.discover_peers()?;

//         // let fp = peers[0].handshake(&torrent_file)?;

//         // fp.read_until(MessageKind::Bitfield(vec![]))?;

//         // fp.send_message(MessageKind::Interested)?;

//         // fp.read_until(MessageKind::Unchoke)?;

//         // let piece = fp.download_piece(piece_index)?;

//         // let mut f = std::fs::File::create(output_file)?;

//         // f.write_all(&piece)?;

//         println!("Piece {} downloaded to {}.", piece_index, output_file);
//     } else {
//         println!("unknown command: {}", args[1])
//     }
// }
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use serde_bencode::value::Value as BencodeValue;
use serde_json::Value as JsonValue;
use sha1::{Digest, Sha1};

#[derive(Debug, Serialize, Deserialize)]
struct Torrent {
    announce: String,
    #[serde(rename = "created by")]
    created_by: String,
    info: Info,
}
#[derive(Debug, Serialize, Deserialize)]
struct Info {
    length: u64,
    name: String,
    #[serde(rename = "piece length")]
    piece_length: i64,
    #[serde(with = "serde_bytes")]
    pieces: Vec<u8>,
}

impl Info {
    fn info_hash(self) -> String {
        let hash = Sha1::digest(serde_bencode::to_bytes(&self).unwrap());
        hash.iter().map(|b| format!("{:x}", b)).collect()
    }
}

#[derive(Parser, Debug)]
#[command(author="Sayan Mallick", version="0.1", about="A simple torrent client written in Rust", long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Decodes Bencode
    Decode { value: String },
    /// Get torrent info
    Info { file: String },
}

fn decode(value: &[u8]) -> JsonValue {
    if let Ok(v) = serde_bencode::from_bytes::<BencodeValue>(value) {
        bencode_to_json(&v)
    } else {
        panic!("Unhandled encoded value");
    }
}
fn bencode_to_json(v: &BencodeValue) -> JsonValue {
    match v {
        BencodeValue::Bytes(b) => {
            JsonValue::String(b.iter().map(|b| *b as char).collect::<String>())
        }
        BencodeValue::Int(i) => JsonValue::Number((*i).into()),
        BencodeValue::List(l) => JsonValue::Array(l.iter().map(bencode_to_json).collect()),
        BencodeValue::Dict(d) => JsonValue::Object(
            d.iter()
                .filter_map(|(k, v)| {
                    String::from_utf8(k.clone())
                        .ok()
                        .map(|ks| (ks, bencode_to_json(&v.clone())))
                })
                .collect(),
        ),
    }
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Decode { value } => {
            let value = decode(value.as_bytes());
            println!("{}", value);
        }
        Commands::Info { file } => {
            let file = std::fs::read(file).unwrap();
            let torrent = decode(&file);
            let torrent: Torrent = serde_json::from_value(torrent).unwrap();
            println!("Tracker URL: {}", torrent.announce);
            println!("Tracker URL: {}", torrent.info.length);
            println!("Tracker URL: {}", torrent.info.info_hash());
        }
    }
}
