use rand::Rng;

pub fn generate_random_message(port: u16) -> String {
    let messages = vec![
        "Hello, world!",
        "Rust is great!",
        "Gossiping in P2P network!",
        "Random message",
        "Peer-to-peer communication",
    ];
    let mut rng = rand::thread_rng();
    let message = messages[rng.gen_range(0..messages.len())];
    format!("{} (from port {})", message, port)
}
