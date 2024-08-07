use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::{Arc, Mutex};

pub fn request_peer_list(peers: Arc<Mutex<Vec<String>>>, address: &str) {
    if let Ok(mut stream) = TcpStream::connect(address) {
        stream.write_all(b"Requesting peer list").expect("Failed to send request");
        let mut buffer = [0; 1024];
        let bytes_read = stream.read(&mut buffer).expect("Failed to read from peer");
        let response = String::from_utf8_lossy(&buffer[..bytes_read]);
        println!("Received peer list: {}", response);
        let new_peers: Vec<String> = response.split(", ")
            .map(|s| s.to_string())
            .collect();
        let mut peers = peers.lock().unwrap();
        for peer in new_peers {
            if !peers.contains(&peer) {
                peers.push(peer);
            }
        }
    }
}
