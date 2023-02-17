use common_types::PacketType;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;
use std::thread;
use std::time::Duration;

fn ping_server() {
    match TcpStream::connect("127.0.0.1:9231") {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 9321");

            let msg = PacketType::TEXT;

            stream.write(&[msg]).unwrap();
            println!("Sent Hello, awaiting reply...");

            let mut data = [0 as u8; 2]; //UNSAFE
            match stream.read(&mut data) {
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

fn main() {
    for _ in 0..5 {
        thread::spawn(|| {
            ping_server();
        });
        thread::sleep(Duration::from_secs(2));
    }
}
