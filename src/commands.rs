use std::net::Ipv4Addr;

use clap::{Args, Parser, Subcommand};

use crate::client::Client;
use crate::server::Server;

/// 💬 TermTalk — Chat for communication in the terminal.
///
/// Usage example:
///
/// 1. Server: termtalk create --port 8080 --username Creator
///
/// 2. Client: termtalk connect 192.168.1.5 --port 8080 --username User
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

    /// Username for chat (default, Creator)
    #[arg(long, short, default_value_t = String::from("Creator"))]
    username: String,
}

/// Connecting to the server
#[derive(Args)]
struct ConnectArgs {
    /// Server IPv4 address (for example: 192.168.1.5)
    host: Ipv4Addr,

    /// Server port (default: 8080)
    #[arg(long, short, default_value_t = 8080)]
    port: u16,

    /// Username for chat (default, User)
    #[arg(long, short, default_value_t = String::from("User"))]
    username: String,
}

pub fn init_commands() {
    let cli = Cli::parse();

    match cli.commands {
        Commands::Create(data) => {
            let server = Server::new(data.port, data.username);
            server.create();
        }
        Commands::Connect(data) => {
            let client = Client::new(data.host, data.port, data.username);
            client.create();
        }
    };
}
