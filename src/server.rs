use crate::chat;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener};

pub fn create_server(port: u16) {
    let host = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), port);

    match TcpListener::bind(host) {
        Ok(listener) => {
            println!("Server created and listening on host {}", host);

            match listener.accept() {
                Ok((stream, addr)) => {
                    println!("Client connecting from IP {}", addr);
                    chat::start_chat(stream);
                }
                Err(e) => println!("Error accepting connection: {}", e),
            };
        }
        Err(e) => println!("Error to create server: {}", e),
    };
}
