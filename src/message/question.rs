use super::error::{MessageError, Result};
use super::name::{decode_name, encode_name};

#[derive(Debug, PartialEq, Eq)]
pub struct Question {
    pub name: String,
    pub qtype: u16,
    pub qclass: u16,
}

impl Question {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = encode_name(&self.name);
        bytes.extend_from_slice(&self.qtype.to_be_bytes());
        bytes.extend_from_slice(&self.qclass.to_be_bytes());
        bytes
    }

    pub fn from_bytes(bytes: &[u8], offset: usize) -> Result<(Self, usize)> {
        let (name, offset) = decode_name(bytes, offset)?;

        let type_and_class = bytes
            .get(offset..offset + 4)
            .ok_or(MessageError::QuestionSectionTruncated)?;
        let qtype = u16::from_be_bytes([type_and_class[0], type_and_class[1]]);
        let qclass = u16::from_be_bytes([type_and_class[2], type_and_class[3]]);

        Ok((
            Question {
                name,
                qtype,
                qclass,
            },
            offset + 4,
        ))
    }
}
