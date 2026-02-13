// Looking at: https://doc.rust-lang.org/std/net/struct.TcpListener.html
use std::io;
use std::io::{Read, Result, Write};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) -> Result<()> {
    let mut data = vec![0u8; 256];
    loop {
        let n = match stream.read(&mut data) {
            Ok(0) => {
                print!("Gracefully closing the connection");
                break;
            }
            Ok(n) => n,
            Err(e) if e.kind() == io::ErrorKind::TimedOut => {
                println!("Client idle, closing connection");
                break;
            }
            Err(e) => return Err(e), // Propagate other errors (like ?)
        };

        // Write - simple ? since we don't care about specific errors
        stream.write_all(&data[..n])?;
    }
    Ok(())
}

fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:18888")?;

    // accept connections and process them serially
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr()?);
                handle_client(stream)?;
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
    Ok(())
}
