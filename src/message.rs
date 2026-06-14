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
pub struct Question {
    pub name: String,
    pub qtype: u16,
    pub qclass: u16,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Message {
    pub header: Header,
    pub questions: Vec<Question>,
}

impl Question {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        for label in self.name.split('.') {
            bytes.push(label.len() as u8);
            bytes.extend_from_slice(label.as_bytes());
        }
        bytes.push(0);
        bytes.extend_from_slice(&self.qtype.to_be_bytes());
        bytes.extend_from_slice(&self.qclass.to_be_bytes());
        bytes
    }
}

impl Header {
    pub fn to_bytes(&self) -> [u8; 12] {
        let mut flags: u16 = 0;
        flags |= (self.qr as u16) << 15;
        flags |= (self.opcode as u16 & 0x0F) << 11;
        flags |= (self.aa as u16) << 10;
        flags |= (self.tc as u16) << 9;
        flags |= (self.rd as u16) << 8;
        flags |= (self.ra as u16) << 7;
        flags |= (self.z as u16 & 0x07) << 4;
        flags |= self.rcode as u16 & 0x0F;

        let mut bytes = [0u8; 12];
        bytes[0..2].copy_from_slice(&self.id.to_be_bytes());
        bytes[2..4].copy_from_slice(&flags.to_be_bytes());
        bytes[4..6].copy_from_slice(&self.qdcount.to_be_bytes());
        bytes[6..8].copy_from_slice(&self.ancount.to_be_bytes());
        bytes[8..10].copy_from_slice(&self.nscount.to_be_bytes());
        bytes[10..12].copy_from_slice(&self.arcount.to_be_bytes());
        bytes
    }

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
        Ok(Message {
            header,
            questions: Vec::new(),
        })
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = self.header.to_bytes().to_vec();
        for question in &self.questions {
            bytes.extend(question.to_bytes());
        }
        bytes
    }
}
