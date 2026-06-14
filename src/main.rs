mod error;
mod message;

use crate::error::{RequestError, ServerError};
use crate::message::{Answer, Header, Message, Question};
use std::net::{SocketAddr, UdpSocket};
use std::process::ExitCode;

fn main() -> ExitCode {
    // Store the resolver address (the value after --resolver) or none if not present
    let args: Vec<String> = std::env::args().collect();
    let resolver = args
        .iter()
        .position(|arg| arg == "--resolver")
        .and_then(|i| args.get(i + 1))
        .cloned();

    match run(resolver) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{err}");
            ExitCode::FAILURE
        }
    }
}

fn run(resolver: Option<String>) -> Result<(), ServerError> {
    println!("Logs from your program will appear here!");

    let udp_socket = UdpSocket::bind("127.0.0.1:2053")?;

    let mut client_buf = [0; 512];
    let mut resolver_buf = [0; 512];

    loop {
        let (size, source) = udp_socket.recv_from(&mut client_buf)?;
        println!("Received {} bytes from {}", size, source);

        if let Err(err) = handle_request(
            &udp_socket,
            &resolver,
            &client_buf[..size],
            source,
            &mut resolver_buf,
        ) {
            eprintln!("Failed to handle request from {source}: {err}");
        }
    }
}

fn handle_request(
    udp_socket: &UdpSocket,
    resolver: &Option<String>,
    request_bytes: &[u8],
    source: SocketAddr,
    resolver_buf: &mut [u8],
) -> Result<(), RequestError> {
    let request = Message::from_bytes(request_bytes);
    if let Err(err) = &request {
        eprintln!("Failed to parse DNS message: {err}");
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

    let answers = match resolver {
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
                        opcode,
                        rd,
                        qdcount: 1,
                        ..Default::default()
                    },
                    questions: vec![Question {
                        name: question.name.clone(),
                        qtype: question.qtype,
                        qclass: question.qclass,
                    }],
                    answers: vec![],
                }
                .to_bytes();

                udp_socket.send_to(&query, resolver_addr)?;
                let (resolver_size, _resolver_source) = udp_socket.recv_from(resolver_buf)?;

                let resolver_response = Message::from_bytes(&resolver_buf[..resolver_size])?;
                answers.extend(resolver_response.answers);
            }
            answers
        }
    };

    let response_header = Header {
        id,
        qr: true,
        opcode,
        rd,
        rcode,
        qdcount: questions.len() as u16,
        ancount: answers.len() as u16,
        ..Default::default()
    };
    let response = Message {
        header: response_header,
        questions,
        answers,
    }
    .to_bytes();

    udp_socket.send_to(&response, source)?;
    Ok(())
}
