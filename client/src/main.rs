use std::io::{Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;
use CommonTypes::PacketType;

fn main() {
    match TcpStream::connect("127.0.0.1:9231") {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 9321");

            let msg = &[PacketType::HELLO];

            stream.write(msg).unwrap();
            println!("Sent Hello, awaiting reply...");

            let mut data = [0 as u8; 2]; //UNSAFE
            match stream.read_exact(&mut data) {
                Ok(_) => {
                    if data == [PacketType::HELLO_ACK as u8; 2] {
                        println!("Reply is ok!");
                    } else {
                        let text = from_utf8(&data).unwrap();
                        println!("Unexpected reply: {}", text);
                    }
                }
                Err(e) => {
                    println!("Failed to receive data: {}", e);
                }
            }
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    println!("Terminated.");
}
