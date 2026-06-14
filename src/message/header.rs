use super::error::{MessageError, Result};

pub(super) const HEADER_SIZE: usize = 12;

// Bit positions of each flag within the 16-bit header flags field (RFC 1035 4.1.1).
const QR_BIT: u16 = 15;
const OPCODE_SHIFT: u16 = 11;
const AA_BIT: u16 = 10;
const TC_BIT: u16 = 9;
const RD_BIT: u16 = 8;
const RA_BIT: u16 = 7;
const Z_SHIFT: u16 = 4;

const SINGLE_BIT_MASK: u16 = 0b1;
const OPCODE_MASK: u16 = 0b1111;
const Z_MASK: u16 = 0b111;
const RCODE_MASK: u16 = 0b1111;

#[derive(Debug, PartialEq, Eq, Default)]
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

impl Header {
    pub fn to_bytes(&self) -> [u8; HEADER_SIZE] {
        let mut flags: u16 = 0;
        flags |= (self.qr as u16) << QR_BIT;
        flags |= (self.opcode as u16 & OPCODE_MASK) << OPCODE_SHIFT;
        flags |= (self.aa as u16) << AA_BIT;
        flags |= (self.tc as u16) << TC_BIT;
        flags |= (self.rd as u16) << RD_BIT;
        flags |= (self.ra as u16) << RA_BIT;
        flags |= (self.z as u16 & Z_MASK) << Z_SHIFT;
        flags |= self.rcode as u16 & RCODE_MASK;

        let mut bytes = [0u8; HEADER_SIZE];
        bytes[0..2].copy_from_slice(&self.id.to_be_bytes());
        bytes[2..4].copy_from_slice(&flags.to_be_bytes());
        bytes[4..6].copy_from_slice(&self.qdcount.to_be_bytes());
        bytes[6..8].copy_from_slice(&self.ancount.to_be_bytes());
        bytes[8..10].copy_from_slice(&self.nscount.to_be_bytes());
        bytes[10..12].copy_from_slice(&self.arcount.to_be_bytes());
        bytes
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < HEADER_SIZE {
            return Err(MessageError::HeaderTooShort(HEADER_SIZE));
        }

        let id = u16::from_be_bytes([bytes[0], bytes[1]]);
        let flags = u16::from_be_bytes([bytes[2], bytes[3]]);

        let qr = (flags >> QR_BIT) & SINGLE_BIT_MASK == 1;
        let opcode = ((flags >> OPCODE_SHIFT) & OPCODE_MASK) as u8;
        let aa = (flags >> AA_BIT) & SINGLE_BIT_MASK == 1;
        let tc = (flags >> TC_BIT) & SINGLE_BIT_MASK == 1;
        let rd = (flags >> RD_BIT) & SINGLE_BIT_MASK == 1;
        let ra = (flags >> RA_BIT) & SINGLE_BIT_MASK == 1;
        let z = ((flags >> Z_SHIFT) & Z_MASK) as u8;
        let rcode = (flags & RCODE_MASK) as u8;

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
