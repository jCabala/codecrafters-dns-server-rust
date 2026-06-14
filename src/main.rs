mod message;

use std::net::UdpSocket;
use crate::message::{Header, Message};

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

                let response_header = Header {
                    id: 1234,
                    qr: true,
                    opcode: 0,
                    aa: false,
                    tc: false,
                    rd: false,
                    ra: false,
                    z: 0,
                    rcode: 0,
                    qdcount: 0,
                    ancount: 0,
                    nscount: 0,
                    arcount: 0,
                };
                let response = Message {
                    header: response_header,
                }
                .to_bytes();

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
