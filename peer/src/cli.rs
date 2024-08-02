use clap::Parser;

/// Command line arguments structure
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Set the period
    #[arg(long)]
    pub period: u32,

    /// Set the port (default is 127.0.0.1)
    #[arg(long, default_value = "")]
    pub connection: String,

    /// Set the port
    #[arg(long, default_value_t = 8080)] // Default port value
    pub port: u16,
}

pub fn get_args() -> Args {
    Args::parse()
}
