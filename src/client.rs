use crate::chat;
use colored::Colorize;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream};

pub fn create_client(ip: Ipv4Addr, port: u16) {
    let host = SocketAddr::new(IpAddr::V4(ip), port);
    println!("Connecting to host {} ...", host);

    match TcpStream::connect(host) {
        Ok(stream) => {
            println!("{}", "Connection established".green().bold());
            chat::start_chat(stream);
        }
        Err(e) => println!(
            "{}",
            format!("Error to connect host {}: {}", host, e)
                .red()
                .bold()
        ),
    };
}
