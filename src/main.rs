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

                let questions: Vec<Question> = request
                    .as_ref()
                    .map(|m| {
                        m.questions
                            .iter()
                            .map(|q| Question {
                                name: q.name.clone(),
                                qtype: q.qtype,
                                qclass: q.qclass,
                            })
                            .collect()
                    })
                    .unwrap_or_default();

                let answers: Vec<Answer> = questions
                    .iter()
                    .map(|q| Answer {
                        name: q.name.clone(),
                        rtype: 1,
                        rclass: 1,
                        ttl: 60,
                        rdata: vec![8, 8, 8, 8],
                    })
                    .collect();

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
                    qdcount: questions.len() as u16,
                    ancount: answers.len() as u16,
                    nscount: 0,
                    arcount: 0,
                };
                let response = Message {
                    header: response_header,
                    questions,
                    answers,
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
