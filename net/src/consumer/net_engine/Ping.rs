use bytes::Bytes;
use log::info;
use protocol::message::message::Message;
use crate::consumer::consume::Consume;

pub struct Ping;

impl From<Bytes> for Ping {
    fn from(_: Bytes) -> Self {
        Ping
    }
}

impl Consume for Ping {
    fn consume(&self, message: Message) {
        info!("Ping!");
    }
}