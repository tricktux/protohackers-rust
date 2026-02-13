// Looking at: https://doc.rust-lang.org/std/net/struct.TcpListener.html
use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) {
    let mut data = vec![0u8; 256];
    loop {
        match stream.read(&mut data) {
            Ok(0) => break, // Connection closed
            Ok(_) => {
                // Write the data back
                // Implement echo
                match stream.write_all(&data) {
                    Ok(_) => continue, // Successfully echoed back, wait for more data
                    Err(e) => {
                        println!("Error writing to stream: {}", e);
                        break; // Exit on write error
                    }
                }
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => continue, // No data available, try again
            Err(e) => panic!("IO error on stream.read: {}", e),              // Handle other errors
        }
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:80")?;

    // accept connections and process them serially
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr()?);
                handle_client(stream);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
    Ok(())
}
