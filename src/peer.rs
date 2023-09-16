use serde::{Deserialize, Serialize};

use crate::torrent::Torrent;

#[derive(Serialize, Deserialize, Debug)]
struct TrackerResponse {
    interval: u64,

    #[serde(with = "serde_bytes")]
    peers: Vec<u8>,
}

pub async fn get_peers(id: &str, torrent: &Torrent) -> Vec<String> {
    let port = 6881;
    let uploaded = 0;
    let downloaded = 0;
    let left = torrent.info.length;
    let compact = 1;

    let url = format!(
        "{}?info_hash={}&peer_id={}&port={}&uploaded={}&downloaded={}&left={}&compact={}",
        torrent.announce,
        torrent.info.url_encoded_hash(),
        id,
        port,
        uploaded,
        downloaded,
        left,
        compact
    );

    let resp = reqwest::get(&url).await.unwrap().bytes().await.unwrap();
    let resp: TrackerResponse = serde_bencode::from_bytes(&resp).unwrap();

    resp.peers
        .chunks(6)
        .map(|chunk| {
            let ip = chunk[0..4]
                .iter()
                .map(|b| format!("{}", b))
                .collect::<Vec<String>>()
                .join(".");
            let port = u16::from_be_bytes([chunk[4], chunk[5]]);
            format!("{}:{}", ip, port)
        })
        .collect()
}
