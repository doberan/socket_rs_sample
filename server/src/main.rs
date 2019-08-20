use std::io::{Error, Read, Write};
use std::net::{TcpListener, TcpStream, Shutdown};
use std::thread;

fn main() {
    let listener = TcpListener::bind("localhost:3000").expect("Error. failed to bind.");
    for streams in listener.incoming() {
        match streams {
            Err(e) => { eprintln!("error: {}", e)},
            Ok(stream) => {
                thread::spawn(move || {
                    handler(stream);
                });
            }
        }
    }
    drop(listener);
}

fn handler(mut stream: TcpStream) {
    println!("Connection from {}", stream.peer_addr().unwrap());
    let mut buffer = [0; 1024];
    while match stream.read(&mut buffer) {
        Ok(nbytes) => {
            println!("nbytes = {:?}", nbytes.to_string());
            stream.write(&buffer[..nbytes]).unwrap();
            true
        },
        Err(_) => {
            println!("Error occured, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {};
}