use std::net::TcpListener;

use colored::Colorize;

use crate::chat::Chat;
use crate::host::Host;

pub struct Server {
    port: u16,
    username: String,
}

impl Server {
    pub fn new(port: u16, username: String) -> Self {
        Self { port, username }
    }

    pub fn create(&self) {
        let host = Host::new(self.port);

        match TcpListener::bind(host.get_broadcast()) {
            Ok(listener) => {
                println!(
                    "{}",
                    format!(
                        "Server created by {} and listening on external host {}",
                        self.username,
                        host.get_external()
                    )
                    .green()
                    .bold()
                );

                println!(
                    "{}",
                    format!(
                        "Server created by {} and listening on internal host {}",
                        self.username,
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
                        let chat = Chat::new(stream, self.username.clone());
                        chat.create();
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
}
