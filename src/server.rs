use std::net::TcpListener;

pub fn create_server(port: u16) {
    match TcpListener::bind(format!("0.0.0.0:{}", port)) {
        Ok(listener) => {
            println!(
                "Server created and listening on host {}:{}",
                "0.0.0.0", port
            );

            match listener.accept() {
                Ok((_, addr)) => println!("Client connecting from IP {}", addr),
                Err(e) => println!("Error accepting connection: {}", e),
            };
        }
        Err(e) => println!("Error to create server: {}", e),
    };
}
