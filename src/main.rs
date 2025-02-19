use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::str;

fn main() -> std::io::Result<()> {
    const PORT: &str = "7890";
    let listener = TcpListener::bind(format!("0.0.0.0:{}", PORT))?;

    println!("Server listening on port {}", PORT);

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                handle_client(&mut stream)?;
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
    Ok(())
}

fn handle_client(stream: &mut TcpStream) -> std::io::Result<()> {
    // Get the client's IP address and port
    let peer_addr = stream.peer_addr()?;
    println!("Server: got connection from {} port {}", peer_addr.ip(), peer_addr.port());

    // Send "Hello World!" to the client
    stream.write_all(b"Hello World!\n")?;

    let mut buffer = [0; 1024];
    loop {
        let bytes_read = stream.read(&mut buffer)?;
        if bytes_read == 0 {
            // Connection closed by client
            break;
        }
        println!("RECV: {} bytes", bytes_read);

        // Convert the buffer to a string slice
        let s = match str::from_utf8(&buffer[..bytes_read]) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Invalid UTF-8 sequence: {}", e);
                break;
            }
        };

        // Here you would typically process the received data.
        // For demonstration, we'll just print it:
        println!("Received: {}", s);
    }
    Ok(())
}