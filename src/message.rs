use anyhow::{anyhow, Result};

#[derive(Debug, PartialEq, Eq)]
pub struct Header {
    pub id: u16,
    pub qr: bool,
    pub opcode: u8,
    pub aa: bool,
    pub tc: bool,
    pub rd: bool,
    pub ra: bool,
    pub z: u8,
    pub rcode: u8,
    pub qdcount: u16,
    pub ancount: u16,
    pub nscount: u16,
    pub arcount: u16,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Message {
    pub header: Header,
}

impl Header {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < 12 {
            return Err(anyhow!("DNS header must be at least 12 bytes long"));
        }

        let id = u16::from_be_bytes([bytes[0], bytes[1]]);
        let flags = u16::from_be_bytes([bytes[2], bytes[3]]);

        let qr = (flags >> 15) & 0x1 == 1;
        let opcode = ((flags >> 11) & 0x0F) as u8;
        let aa = (flags >> 10) & 0x1 == 1;
        let tc = (flags >> 9) & 0x1 == 1;
        let rd = (flags >> 8) & 0x1 == 1;
        let ra = (flags >> 7) & 0x1 == 1;
        let z = ((flags >> 4) & 0x07) as u8;
        let rcode = (flags & 0x0F) as u8;

        let qdcount = u16::from_be_bytes([bytes[4], bytes[5]]);
        let ancount = u16::from_be_bytes([bytes[6], bytes[7]]);
        let nscount = u16::from_be_bytes([bytes[8], bytes[9]]);
        let arcount = u16::from_be_bytes([bytes[10], bytes[11]]);

        Ok(Header {
            id,
            qr,
            opcode,
            aa,
            tc,
            rd,
            ra,
            z,
            rcode,
            qdcount,
            ancount,
            nscount,
            arcount,
        })
    }
}

impl Message {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let header = Header::from_bytes(bytes)?;
        Ok(Message { header })
    }
}
