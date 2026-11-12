use std::net::Ipv4Addr;

use clap::Parser;

const MAX_LENGTH_USERNAME: usize = 20;
const MIN_LENGTH_USERNAME: usize = 2;

/// Termtalk - terminal-based chat by Sergey Klimov
#[derive(Parser)]
#[command(version, about, long_about = None, override_usage = "termtalk <HOST> --port <PORT> --username <USERNAME> --key <KEY>")]
pub struct Arguments {
    /// Host to connect
    pub host: Ipv4Addr,

    /// Port of the host
    #[arg(short, long, default_value_t = 8000)]
    pub port: u16,

    /// Name of the user
    #[arg(short, long, value_parser = validate_username)]
    pub username: String,

    /// Crypto key
    #[arg(short, long)]
    pub key: String,
}

pub fn create() -> Arguments {
    Arguments::parse()
}

fn validate_username(username: &str) -> Result<String, String> {
    let trimed = username.trim();
    let len = trimed.chars().count();

    if len > MAX_LENGTH_USERNAME {
        Err(format!(
            "Maximum number of characters: {}",
            MAX_LENGTH_USERNAME
        ))
    } else if len < 2 {
        Err(format!(
            "Minimum number of characters: {}",
            MIN_LENGTH_USERNAME
        ))
    } else {
        Ok(trimed.to_string())
    }
}
