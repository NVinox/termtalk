use std::io;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::process;
use std::thread;

use colored::Colorize;

pub struct Chat {
    stream: TcpStream,
    username: String,
}

impl Chat {
    pub fn new(stream: TcpStream, username: String) -> Self {
        Self { stream, username }
    }

    pub fn create(self) {
        let mut stream_for_reading = self.stream.try_clone().expect("Failed to clone stream");
        let mut stream_for_writing = self.stream;

        thread::spawn(move || {
            let mut buffer = [0; 512];

            loop {
                match stream_for_reading.read(&mut buffer) {
                    Ok(0) => {
                        println!("{}", "User is diconnected".red().bold());
                        process::exit(0);
                    }
                    Ok(bytes) => {
                        let message = String::from_utf8_lossy(&buffer[..bytes]);
                        println!("{}", message.trim());
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

            let trimmed = input.trim();
            if trimmed.is_empty() {
                continue;
            }
            let formatted_message = format!("{}: {}\n", self.username, trimmed);

            if stream_for_writing
                .write_all(formatted_message.as_bytes())
                .is_err()
            {
                println!("{}", "Failed to send message".red().bold());
                break;
            }
        }
    }
}
