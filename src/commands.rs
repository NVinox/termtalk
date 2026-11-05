use clap::{Args, Parser, Subcommand};
use std::net::Ipv4Addr;

/// 💬 TermTalk — Chat for communication in the terminal.
///
/// Usage example:
///
/// 1. Server: termtalk create --port 8080
///
/// 2. Client: termtalk connect 192.168.1.5 --port 8080
#[derive(Parser)]
#[command(
    name = "Termtalk",
    version = "0.1.0",
    about = "A program for direct communication between computers via the console"
)]
struct Cli {
    #[command(subcommand)]
    commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Create(CreateArgs),
    Connect(ConnectArgs),
}

/// Creating a server
#[derive(Args)]
struct CreateArgs {
    /// The port on which the server will run (default, 8080)
    #[arg(long, short, default_value_t = 8080)]
    port: u16,
}

/// Connecting to the server
#[derive(Args)]
struct ConnectArgs {
    /// Server IPv4 address (for example: 192.168.1.5)
    host: Ipv4Addr,

    /// Server port (default: 8080)
    #[arg(long, short, default_value_t = 8080)]
    port: u16,
}

pub fn init_commands() {
    let cli = Cli::parse();

    match &cli.commands {
        Commands::Create(data) => {
            println!("Server host is listen on {} port...", data.port);
        }
        Commands::Connect(data) => {
            println!("Connecting to {} on {} port...", data.host, data.port);
        }
    };
}
