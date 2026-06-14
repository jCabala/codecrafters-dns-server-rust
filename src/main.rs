mod message;

use std::net::UdpSocket;
use crate::message::Message;

fn main() {
    println!("Logs from your program will appear here!");

    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let mut buf = [0; 512];

    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                println!("Received {} bytes from {}", size, source);
                match Message::from_bytes(&buf[..size]) {
                    Ok(message) => println!("Parsed DNS header: {:#?}", message.header),
                    Err(err) => eprintln!("Failed to parse DNS message: {}", err),
                }

                let response = [];
                udp_socket
                    .send_to(&response, source)
                    .expect("Failed to send response");
            }
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
                break;
            }
        }
    }
}
