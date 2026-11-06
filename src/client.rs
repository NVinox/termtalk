use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream};

use colored::Colorize;

use crate::chat::Chat;

pub struct Client {
    host: Ipv4Addr,
    port: u16,
    username: String,
}

impl Client {
    pub fn new(host: Ipv4Addr, port: u16, username: String) -> Self {
        Self {
            host,
            port,
            username,
        }
    }

    pub fn create(&self) {
        let host = SocketAddr::new(IpAddr::V4(self.host), self.port);
        println!("Connecting to host {} ...", host);

        match TcpStream::connect(host) {
            Ok(stream) => {
                println!("{}", "Connection established".green().bold());
                let chat = Chat::new(stream, self.username.clone());
                chat.create();
            }
            Err(e) => println!(
                "{}",
                format!("Error to connect host {}: {}", host, e)
                    .red()
                    .bold()
            ),
        };
    }
}
