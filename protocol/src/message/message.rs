use std::fmt::{Display, Formatter};
use crate::message::header::{Header, MAX_HEADER_SIZE};

const MAX_MESSAGE_SIZE: usize = 8192;

pub struct Message {
    pub header: Header,
    pub data: [u8; MAX_MESSAGE_SIZE],
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
            header: self.header.clone(),
        }
    }
}

impl Display for Message {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let a = self.header.id();
        write!(f, "{} ({} byte(s))", a, self.header.data_size())
    }
}

impl From<&[u8]> for Message {
    fn from(data: &[u8]) -> Self {
        let mut buf: [u8; MAX_MESSAGE_SIZE] = [0; MAX_MESSAGE_SIZE];
        buf[..data.len() - 6].copy_from_slice(&data[6..]);

        Message {
            header: Header::from(&data[0..MAX_HEADER_SIZE as usize]),
            data: buf,
        }
    }
}
