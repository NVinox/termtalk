use crate::chat;
use crate::host::Host;

use colored::Colorize;
use std::net::TcpListener;

pub fn create_server(port: u16) {
    let host = Host::new(port);

    match TcpListener::bind(host.get_broadcast()) {
        Ok(listener) => {
            println!(
                "{}",
                format!(
                    "Server created and listening on external host {}",
                    host.get_external()
                )
                .green()
                .bold()
            );

            println!(
                "{}",
                format!(
                    "Server created and listening on internal host {}",
                    host.get_internal()
                )
                .green()
                .bold()
            );

            match listener.accept() {
                Ok((stream, addr)) => {
                    println!(
                        "{}",
                        format!("Client connecting from host {addr}").green().bold()
                    );
                    chat::start_chat(stream);
                }
                Err(e) => println!(
                    "{}",
                    format!("Error accepting connection: {}", e).red().bold()
                ),
            };
        }
        Err(e) => println!("{}", format!("Error to create server: {}", e).red().bold()),
    };
}
