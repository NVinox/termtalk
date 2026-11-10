use std::net::Ipv4Addr;

use clap::Parser;

/// Termtalk - terminal-based chat by Sergey Klimov
#[derive(Parser)]
#[command(version, about, long_about = None, override_usage = "termtalk <HOST> --username <USERNAME>")]
pub struct Arguments {
    /// Host to connect
    pub host: Ipv4Addr,

    /// Name of the user
    #[arg(short, long)]
    pub username: String,
}

pub fn create() -> Arguments {
    Arguments::parse()
}
