mod message;

use std::net::UdpSocket;
use crate::message::{Answer, Header, Message, Question};

fn main() {
    println!("Logs from your program will appear here!");

    // Store the resolver address (the value after --resolver) or none if not present
    let args: Vec<String> = std::env::args().collect();
    let resolver = args
        .iter()
        .position(|arg| arg == "--resolver")
        .and_then(|i| args.get(i + 1))
        .cloned();
    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    
    let mut client_buf = [0; 512];
    let mut resolver_buf = [0; 512];

    loop {
        match udp_socket.recv_from(&mut client_buf) {
            Ok((size, source)) => {
                println!("Received {} bytes from {}", size, source);
                let request = Message::from_bytes(&client_buf[..size]);
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
                
                let answers = match &resolver {
                    None => {
                        // Send dummy answers
                        questions
                            .iter()
                            .map(|q| Answer {
                                name: q.name.clone(),
                                rtype: 1,
                                rclass: 1,
                                ttl: 60,
                                rdata: vec![8, 8, 8, 8],
                            })
                            .collect()
                    }
                    Some(resolver_addr) => {
                        // The resolver only answers single-question queries, so forward each
                        // question separately and merge the answers into one response.
                        let mut answers = Vec::new();
                        for question in &questions {
                            let query = Message {
                                header: Header {
                                    id,
                                    qr: false,
                                    opcode,
                                    aa: false,
                                    tc: false,
                                    rd,
                                    ra: false,
                                    z: 0,
                                    rcode: 0,
                                    qdcount: 1,
                                    ancount: 0,
                                    nscount: 0,
                                    arcount: 0,
                                },
                                questions: vec![Question {
                                    name: question.name.clone(),
                                    qtype: question.qtype,
                                    qclass: question.qclass,
                                }],
                                answers: vec![],
                            }
                            .to_bytes();

                            udp_socket
                                .send_to(&query, resolver_addr)
                                .expect("Failed to send query to resolver");
                            let (resolver_size, _resolver_source) = udp_socket
                                .recv_from(&mut resolver_buf)
                                .expect("Failed to receive message from resolver");

                            let resolver_response =
                                Message::from_bytes(&resolver_buf[..resolver_size])
                                    .expect("Failed to parse resolver response");
                            answers.extend(resolver_response.answers);
                        }
                        answers
                    }
                };


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
