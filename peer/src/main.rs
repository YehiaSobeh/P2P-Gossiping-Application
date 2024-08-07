
mod cli;
mod connection;
mod message;
mod peer;

use std::sync::Arc;
use std::thread;
use std::time::Duration;

use cli::get_args;
use peer::Peer;

fn main() {
    let args = get_args();
    println!(
        "Starting peer with port: {}, period: {}, connection: {:?}",
        args.port, args.period, args.connection
    );

    let period = args.period as u64; // Convert u32 to u64
    let peer = Arc::new(Peer::new(args.port, period));

    if !args.connection.is_empty() {
        peer.connect_to(&args.connection);
    }

    Arc::clone(&peer).start();

    // Keep the main thread alive to allow background threads to run
    loop {
        thread::sleep(Duration::from_secs(1));
    }
}

