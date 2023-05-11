use bytes::Bytes;
use protocol::message::message::Message;

pub trait Consume {
    fn consume(&self, message: Message);
}