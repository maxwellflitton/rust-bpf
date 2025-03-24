use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::convert::TryInto;

fn handle_client(mut stream: TcpStream) {
    println!("New client connected: {:?}", stream.peer_addr());

    // Create a buffer for 4 bytes
    let mut buffer = [0u8; 4];

    loop {
        // Read exactly 4 bytes
        match stream.read_exact(&mut buffer) {
            Ok(_) => {
                // Convert 4 bytes into a u32 (big-endian)
                let number = u32::from_be_bytes(buffer);
                println!("Received number: {}", number);

                // Echo back the same number (as 4 bytes)
                if let Err(e) = stream.write_all(&buffer) {
                    eprintln!("Failed to write to client: {}", e);
                    break;
                }
            }
            Err(e) => {
                eprintln!("Failed to read from client: {}", e);
                break;
            }
        }
    }

    println!("Client disconnected: {:?}", stream.peer_addr());
}

fn main() -> std::io::Result<()> {
    // Bind the server to localhost on port 7878
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    println!("Server listening on 127.0.0.1:7878");

    // Accept incoming connections in a loop
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // Spawn a new thread for each client
                thread::spawn(|| handle_client(stream));
            }
            Err(e) => {
                eprintln!("Failed to accept client: {}", e);
            }
        }
    }

    Ok(())
}
