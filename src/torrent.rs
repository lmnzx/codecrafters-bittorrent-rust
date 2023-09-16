use serde::{Deserialize, Serialize};
use sha1::{Digest, Sha1};

#[derive(Debug, Serialize, Deserialize)]
pub struct Torrent {
    pub announce: String,
    #[serde(rename = "created by")]
    pub created_by: Option<String>,
    pub info: Info,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Info {
    pub length: u64,
    pub name: String,
    #[serde(rename = "piece length")]
    pub piece_length: i64,
    #[serde(with = "serde_bytes")]
    pub pieces: Vec<u8>,
}

impl Info {
    pub fn info_hash(&self) -> String {
        let hash = Sha1::digest(serde_bencode::to_bytes(&self).unwrap());
        hash.iter().map(|b| format!("{:02x}", b)).collect()
    }

    pub fn piece_hashes(&self) -> Vec<String> {
        self.pieces
            .chunks(20)
            .map(|chunk| {
                chunk
                    .iter()
                    .map(|b| format!("{:02x}", b))
                    .collect::<String>()
            })
            .collect()
    }

    pub fn url_encoded_hash(&self) -> String {
        self.info_hash()
            .chars()
            .collect::<Vec<char>>()
            .chunks(2)
            .fold(String::new(), |acc, chuck| {
                let s = chuck.iter().collect::<String>();
                let c = u8::from_str_radix(&s, 16).unwrap();
                match c {
                    5 | 46 | 48..=57 | 65..=90 | 95 | 97..=122 => acc + &char::from(c).to_string(),
                    _ => acc + "%" + &s,
                }
            })
    }
}
