use anyhow::{anyhow, Result};

// A length byte starting with 0b11 marks a compression pointer (RFC 1035 4.1.4);
// the remaining 14 bits across both bytes encode the target offset.
const POINTER_TAG: u8 = 0b1100_0000;
const POINTER_OFFSET_MASK: u8 = 0b0011_1111;

pub fn encode_name(name: &str) -> Vec<u8> {
    let mut bytes = Vec::new();
    for label in name.split('.') {
        bytes.push(label.len() as u8);
        bytes.extend_from_slice(label.as_bytes());
    }
    bytes.push(0);
    bytes
}

pub fn decode_name(bytes: &[u8], start: usize) -> Result<(String, usize)> {
    let mut labels = Vec::new();
    let mut pos = start;
    let mut end_offset = None;

    loop {
        let len_byte = *bytes
            .get(pos)
            .ok_or_else(|| anyhow!("Unexpected end of buffer while parsing name"))?;

        if len_byte & POINTER_TAG == POINTER_TAG {
            let next = *bytes
                .get(pos + 1)
                .ok_or_else(|| anyhow!("Unexpected end of buffer while parsing name pointer"))?;
            if end_offset.is_none() {
                end_offset = Some(pos + 2);
            }

            pos = (((len_byte & POINTER_OFFSET_MASK) as usize) << 8) | next as usize;
            continue;
        }

        let len = len_byte as usize;
        pos += 1;
        if len == 0 {
            break;
        }

        let label = bytes
            .get(pos..pos + len)
            .ok_or_else(|| anyhow!("Label exceeds buffer length"))?;
        labels.push(String::from_utf8_lossy(label).to_string());
        pos += len;
    }

    Ok((labels.join("."), end_offset.unwrap_or(pos)))
}
