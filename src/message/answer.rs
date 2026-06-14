use anyhow::{anyhow, Result};

use super::name::{decode_name, encode_name};

#[derive(Debug, PartialEq, Eq)]
pub struct Answer {
    pub name: String,
    pub rtype: u16,
    pub rclass: u16,
    pub ttl: u32,
    pub rdata: Vec<u8>,
}

impl Answer {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = encode_name(&self.name);
        bytes.extend_from_slice(&self.rtype.to_be_bytes());
        bytes.extend_from_slice(&self.rclass.to_be_bytes());
        bytes.extend_from_slice(&self.ttl.to_be_bytes());
        bytes.extend_from_slice(&(self.rdata.len() as u16).to_be_bytes());
        bytes.extend_from_slice(&self.rdata);
        bytes
    }

    pub fn from_bytes(bytes: &[u8], offset: usize) -> Result<(Self, usize)> {
        let (name, offset) = decode_name(bytes, offset)?;

        let fixed_fields = bytes
            .get(offset..offset + 10)
            .ok_or_else(|| anyhow!("Answer record truncated"))?;
        let rtype = u16::from_be_bytes([fixed_fields[0], fixed_fields[1]]);
        let rclass = u16::from_be_bytes([fixed_fields[2], fixed_fields[3]]);
        let ttl = u32::from_be_bytes([
            fixed_fields[4],
            fixed_fields[5],
            fixed_fields[6],
            fixed_fields[7],
        ]);
        let rdlength = u16::from_be_bytes([fixed_fields[8], fixed_fields[9]]) as usize;
        let offset = offset + 10;

        let rdata = bytes
            .get(offset..offset + rdlength)
            .ok_or_else(|| anyhow!("Answer record RDATA truncated"))?
            .to_vec();

        Ok((
            Answer {
                name,
                rtype,
                rclass,
                ttl,
                rdata,
            },
            offset + rdlength,
        ))
    }
}
