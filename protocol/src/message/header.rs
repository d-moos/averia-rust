use crate::message::message_id::MessageId;

const HEADER_OFFSET: u16 = 0;
pub const MAX_HEADER_SIZE: u16 = 6;
const HEADER_ENC_OFFSET: u16 = HEADER_OFFSET + 2;
const HEADER_ENC_SIZE: u16 = MAX_HEADER_SIZE - HEADER_ENC_OFFSET;
const HEADER_ENC_MASK: u16 = 0x8000;

pub struct Header {
    size: u16,
    id: MessageId,
    sequence: u8,
    checksum: u8,
}

impl Clone for Header {
    fn clone(&self) -> Self {
        Header {
            size: self.size.clone(),
            id: self.id.clone(),
            checksum: self.checksum.clone(),
            sequence: self.sequence.clone()
        }
    }
}

impl From<&[u8]> for Header {
    fn from(buffer: &[u8]) -> Self {
        Header {
            size: u16::from_le_bytes(buffer[0..2].try_into().unwrap()),
            id: MessageId::from(u16::from_le_bytes(buffer[2..2].try_into().unwrap())),
            checksum: buffer[5],
            sequence: buffer[4],
        }
    }
}

impl Header {
    pub fn data_size(&self) -> u16 {
        (self.size & !HEADER_ENC_MASK) + MAX_HEADER_SIZE
    }

    pub fn is_encrypted(&self) -> bool {
        (self.size & HEADER_ENC_MASK) != 0
    }
}