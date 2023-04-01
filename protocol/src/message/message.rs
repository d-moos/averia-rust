use crate::message::header::{Header, MAX_HEADER_SIZE};

const MAX_MESSAGE_SIZE: usize = 8192;

pub struct Message {
    header: Header,
    data: [u8; MAX_MESSAGE_SIZE],
}

impl Message {
    pub fn is_encrypted(&self) -> bool {
        self.header.is_encrypted()
    }
}

impl Clone for Message {
    fn clone(&self) -> Self {
        Message {
            data: self.data.clone(),
            header: self.header.clone()
        }
    }
}

impl From<&[u8]> for Message {
    fn from(data: &[u8]) -> Self {
        Message {
            header: Header::from(&data[0..MAX_HEADER_SIZE as usize]),
            data: data[6..].try_into().unwrap()
        }
    }
}
