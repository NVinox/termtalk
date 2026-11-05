use colored::Colorize;
use std::io;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::process;
use std::thread;

pub fn start_chat(stream: TcpStream) {
    let mut stream_for_reading = stream.try_clone().expect("Failed to clone stream");
    let mut stream_for_writing = stream;

    thread::spawn(move || {
        let mut buffer = [0; 512];

        loop {
            match stream_for_reading.read(&mut buffer) {
                Ok(0) => {
                    println!("{}", "User is disconnected".red().bold());
                    process::exit(0);
                }
                Ok(bytes) => {
                    let message = String::from_utf8_lossy(&buffer[..bytes]);
                    println!("From User: {}", message.trim());
                }
                Err(e) => {
                    println!("{}", format!("Connection error: {e}").red().bold());
                }
            };
        }
    });

    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Error read message. Closing chat...");

        let input = input.trim();

        if stream_for_writing.write_all(input.as_bytes()).is_err() {
            println!("{}", "Failed to send message".red().bold());
            break;
        }
    }
}
