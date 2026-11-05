use crate::chat;
use colored::Colorize;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener};

pub fn create_server(port: u16) {
    let host = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), port);

    match TcpListener::bind(host) {
        Ok(listener) => {
            println!(
                "{}",
                format!("Server created and listening on host {}", host)
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
