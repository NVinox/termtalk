use std::net::Ipv4Addr;

use clap::Parser;

/// Termtalk - terminal-based chat by Sergey Klimov
#[derive(Parser)]
#[command(version, about, long_about = None, override_usage = "termtalk <HOST> --port <PORT> --username <USERNAME>")]
pub struct Arguments {
    /// Host to connect
    pub host: Ipv4Addr,

    /// Port of the host
    #[arg(short, long, default_value_t = 8000)]
    pub port: u16,

    /// Name of the user
    #[arg(short, long)]
    pub username: String,
}

pub fn create() -> Arguments {
    Arguments::parse()
}
