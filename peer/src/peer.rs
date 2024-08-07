use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration};
use chrono::{Local, Timelike};
use crate::connection::request_peer_list;
use crate::message::generate_random_message;

pub struct Peer {
    port: u16,
    period: u64,
    peers: Arc<Mutex<Vec<String>>>,
}

impl Peer {
    pub fn new(port: u16, period: u64) -> Self {
        Self {
            port,
            period,
            peers: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn start(self: Arc<Self>) {
        let address = format!("127.0.0.1:{}", self.port);
        println!("Binding to address: {}", address);
        let listener = TcpListener::bind(&address).expect("Failed to bind to address");
        println!("Peer listening on port {}", self.port);

        let peers_clone = Arc::clone(&self.peers);
        thread::spawn(move || {
            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => {
                        let peers_clone = Arc::clone(&peers_clone);
                        thread::spawn(move || handle_client(stream, peers_clone));
                    }
                    Err(e) => eprintln!("Failed to establish connection: {}", e),
                }
            }
        });

        let period = self.period;
        let peers_clone = Arc::clone(&self.peers);
        let port = self.port; // Capture the port
        thread::spawn(move || loop {
            thread::sleep(Duration::from_secs(period));
            let peers = peers_clone.lock().unwrap();
            let message = generate_random_message(port); // Pass the port
            for peer in peers.iter() {
                if let Ok(mut stream) = TcpStream::connect(peer) {
                    let message = format!("Gossip: {}", message);
                    stream.write_all(message.as_bytes()).expect("Failed to send message");
                    let (hours, minutes, seconds) = print_current_time();
                    println!("{:02}:{:02}:{:02} - Sending message [{}] to {}",hours,minutes,seconds, message, peer);
                }
            }
        });
    }

    pub fn connect_to(&self, address: &str) {
        self.peers.lock().unwrap().push(address.to_string());
        request_peer_list(Arc::clone(&self.peers), address);
    }
}


fn print_current_time() -> (u32, u32, u32) {
    // Get the current local time
    let now = Local::now();

    // Extract hours, minutes, and seconds
    let hours = now.hour();
    let minutes = now.minute();
    let seconds = now.second();
    (hours, minutes, seconds)


 
}

fn handle_client(mut stream: TcpStream, peers: Arc<Mutex<Vec<String>>>) {
    let peer_addr = stream.peer_addr().expect("Failed to get peer address");
 

    print_current_time();
    let (hours, minutes, seconds) = print_current_time();
    println!("{:02}:{:02}:{:02} - Received request from: {}", hours, minutes, seconds, peer_addr);
 

    let mut buffer = [0; 1024];
    let bytes_read = stream.read(&mut buffer).expect("Failed to read from client");
    let request = String::from_utf8_lossy(&buffer[..bytes_read]);

    if request.contains("Requesting peer list") {
        let peers = peers.lock().unwrap();
        let response = peers.join(", ");
        stream.write_all(response.as_bytes()).expect("Failed to write response");
    } else if request.starts_with("Gossip: ") {
        let message = &request["Gossip: ".len()..];
        //print_current_time();
        let (hours, minutes, seconds) = print_current_time();

        println!("{:02}:{:02}:{:02} - Received message [{}] from {}", hours, minutes, seconds, message, peer_addr);
    } else {
        let response = "Hello, Peer!".as_bytes();
        stream.write_all(response).expect("Failed to write response");
        let peer_address = format!("{}", peer_addr);
        peers.lock().unwrap().push(peer_address);
    }
}
