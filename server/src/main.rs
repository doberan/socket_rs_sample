use std::io::{Error, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn main() {
    let listener = TcpListener::bind("localhost:3000").expect("Error. failed to bind.");
    for streams in listener.incoming() {
        match streams {
            Err(e) => { eprintln!("error: {}", e)},
            Ok(stream) => {
                thread::spawn(move || {
                    handler(stream).unwrap_or_else(|error| eprintln!("{:?}", error));
                });
            }
        }
    }
}

fn handler(mut stream: TcpStream) -> Result<(), Error> {
    println!("Connection from {}", stream.peer_addr()?);
    let mut buffer = [0; 1024];
    loop {
        let nbytes = stream.read(&mut buffer)?;
        if nbytes == 0 {
            return Ok(());
        }
        stream.write(&buffer[..nbytes])?;
        stream.flush()?;
    }
}