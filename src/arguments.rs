use std::net::Ipv4Addr;

use clap::Parser;

const MAX_LENGTH_USERNAME: usize = 20;
const MIN_LENGTH_USERNAME: usize = 2;

/// Termtalk - Lightweight, end-to-end encrypted terminal chat written in Rust
#[derive(Parser)]
#[command(version, about, long_about = None, override_usage = "termtalk <HOST> --port <PORT> --username <NAME> --key <SECRET>")]
pub struct Arguments {
    /// Host to connect
    pub host: Ipv4Addr,

    /// Port of the host
    #[arg(short, long)]
    pub port: u16,

    /// Display name in chat (validated: 2 to 20 characters)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_username_short_is_err() {
        assert!(validate_username("u").is_err());
    }

    #[test]
    fn validate_username_long_is_err() {
        let username = "u".repeat(21);
        assert!(validate_username(&username).is_err());
    }

    #[test]
    fn validate_username_is_ok() {
        let username = "u".repeat(5);
        assert!(validate_username(&username).is_ok());
    }

    #[test]
    fn validate_username_min_is_ok() {
        let username = "u".repeat(MIN_LENGTH_USERNAME);
        assert!(validate_username(&username).is_ok());
    }

    #[test]
    fn validate_username_max_is_ok() {
        let username = "u".repeat(MAX_LENGTH_USERNAME);
        assert!(validate_username(&username).is_ok());
    }

    #[test]
    fn validate_username_trim_is_ok() {
        let username = "    uu ";
        assert_eq!(validate_username(username).unwrap(), "uu");
    }

    #[test]
    fn username_counts_chars_not_bytes() {
        let username = "ё".repeat(2);
        assert!(validate_username(&username).is_ok());
    }
}
