use clap::{Parser, Subcommand};
use serde_bencode::from_bytes;

const PEER_ID: &str = "00112233445566778899";

mod decode;
mod peer;
mod torrent;

use crate::decode::decode;
use crate::torrent::Torrent;

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
    /// Get peer list
    Peers { file: String },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Decode { value } => {
            let value = decode(value.as_bytes());
            println!("{}", value);
        }
        Commands::Info { file } => {
            let file = std::fs::read(file).unwrap();
            let torrent: Torrent = from_bytes(&file).unwrap();
            println!("Tracker URL: {}", torrent.announce);
            println!("Length: {}", torrent.info.length);
            println!("Info Hash: {}", torrent.info.info_hash());
            println!("Piece Length: {}", torrent.info.piece_length);
            println!("Piece Hashes:\n{}", torrent.info.piece_hashes().join("\n"));
        }
        Commands::Peers { file } => {
            let file = std::fs::read(file).unwrap();
            let torrent: Torrent = from_bytes(&file).unwrap();
            let peers = peer::get_peers(PEER_ID, &torrent).await;
            println!("Peers:\n{}", peers.join("\n"));
        }
    }
}
