use std::thread;
use std::{
    io::prelude::*,
    net::{Shutdown, TcpListener, TcpStream},
};
use CommonTypes::PacketType;

fn handle_client(mut stream: TcpStream) {
    let mut data = [0 as u8; 50]; //UNSAFE FIX LATER
    let response = [PacketType::HELLO_ACK as u8; 50];
    while if let Ok(_size) = stream.read(&mut data) {
        //Send HELLO_ACK packet to acknowledge we recieved the packet
        stream.write(&response).unwrap();
        true
    } else {
        println!(
            "An error occurred, terminating connection with {}",
            stream.peer_addr().unwrap()
        );
        stream.shutdown(Shutdown::Both).unwrap();
        false
    } {}
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:9231")?;

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        thread::spawn(|| {
            handle_client(stream);
        });
    }
    Ok(())
}
