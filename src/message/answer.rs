use super::name::encode_name;

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
}
