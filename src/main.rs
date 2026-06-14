mod message;

use std::net::UdpSocket;
use crate::message::{Answer, Header, Message, Question};

fn main() {
    println!("Logs from your program will appear here!");

    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let mut buf = [0; 512];

    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                println!("Received {} bytes from {}", size, source);
                let request = Message::from_bytes(&buf[..size]);
                if let Err(err) = &request {
                    eprintln!("Failed to parse DNS message: {}", err);
                }

                let request_header = request.as_ref().map(|m| &m.header);
                let id = request_header.map_or(1234, |h| h.id);
                let opcode = request_header.map_or(0, |h| h.opcode);
                let rd = request_header.map_or(false, |h| h.rd);
                let rcode = if opcode == 0 { 0 } else { 4 };

                let response_header = Header {
                    id,
                    qr: true,
                    opcode,
                    aa: false,
                    tc: false,
                    rd,
                    ra: false,
                    z: 0,
                    rcode,
                    qdcount: 1,
                    ancount: 1,
                    nscount: 0,
                    arcount: 0,
                };
                let response = Message {
                    header: response_header,
                    questions: vec![Question {
                        name: "codecrafters.io".to_string(),
                        qtype: 1,
                        qclass: 1,
                    }],
                    answers: vec![Answer {
                        name: "codecrafters.io".to_string(),
                        rtype: 1,
                        rclass: 1,
                        ttl: 60,
                        rdata: vec![8, 8, 8, 8],
                    }],
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
