use std::net::{ToSocketAddrs, SocketAddr, TcpStream};
use std::io::{BufRead, Write, BufReader, BufWriter};

fn main() {
    let host = "localhost";
    let port = 3000;
    let host_and_port = format!("{}:{}", host, port);
    let mut addrs = host_and_port.to_socket_addrs().unwrap();
    if let Some(addr) = addrs.find(|x| (*x).is_ipv4()) {
        match TcpStream::connect(addr) {
            Err(_) => {
                println!("Connection NG.");
            }
            Ok(stream) => {
                println!("Connection Ok.");
                // read, write
                let mut reader = BufReader::new(&stream);
                let mut writer = BufWriter::new(&stream);

                read_something(&mut reader);
                write_something(&mut writer, "test message");
            }
        }
    } else {
        eprintln!("Invalid Host:Port Number");
    }
}

fn read_something (reader: &mut BufReader<&TcpStream>) {
  let mut msg = String::new();
  reader.read_line(&mut msg).expect("RECEIVE FAILURE!!!");
  println!("{}", msg);
}

fn write_something (writer: &mut BufWriter<&TcpStream>, comment: &str) {
  let msg = format!("MESSAGE: {}\n", comment);
  writer.write(msg.as_bytes()).expect("SEND FAILURE!!!");
  writer.flush().unwrap();
}