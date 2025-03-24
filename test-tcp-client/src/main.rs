use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Duration;

fn main() -> std::io::Result<()> {
    // Connect to the TCP server
    let mut stream = TcpStream::connect("127.0.0.1:7878")?;
    println!("Connected to the server!");

    // Optional: set a read timeout (just for demo purposes)
    stream.set_read_timeout(Some(Duration::from_secs(5)))?;

    // The integer we want to send
    let number_to_send: u32 = 42;

    // Convert the u32 into 4 bytes in big-endian order
    let bytes = number_to_send.to_be_bytes();

    // Send the 4 bytes to the server
    stream.write_all(&bytes)?;
    println!("Sent number: {}", number_to_send);

    // Create a buffer to receive 4 bytes
    let mut buffer = [0u8; 4];

    // Read exactly 4 bytes as the echo response
    stream.read_exact(&mut buffer)?;

    // Convert the received bytes back to a u32
    let echoed_number = u32::from_be_bytes(buffer);
    println!("Received echoed number: {}", echoed_number);

    Ok(())
}
