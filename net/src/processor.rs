use std::collections::HashMap;
use std::hash::Hash;
use log::{info, log, trace};
use tokio::sync::mpsc::Receiver;
use protocol::message::message::Message;
use protocol::message::message_type::MessageType;
use crate::consumer::consume::Consume;
use crate::consumer::net_engine::Ping::Ping;

// Goal:
// - Consume Net Engine Packets internally; do not dispatch further
// - Encrypt Messages before sending
// - Abstract Massive Messages (trait?)

// requires security context
// must have a possibility to send packets


type MessageTable = HashMap<u16, Box<dyn Consume>>;

pub struct NetProcessor {
    net_engine_table: MessageTable,
    routing_table: MessageTable,
    rx: Receiver<Message>,
}


impl NetProcessor {
    pub fn new(rx: Receiver<Message>, routing_table: MessageTable) -> Self {
        let mut net_engine_table: MessageTable = HashMap::new();

        net_engine_table.insert(0x5000, Box::new(Ping));

        NetProcessor {
            net_engine_table,
            routing_table,
            rx
        }
    }

    pub async fn run(mut self) {
        loop {
            let m = self.rx.recv().await.unwrap();

            // decrypt message
            let was_encrypted = m.is_encrypted();
            if m.is_encrypted() {}

            // massive check
            // IF IS_MASSIVE

            info!("📥 {}", m);

            match m.header.id().message_type {
                MessageType::NetEngine => {
                    let id: u16 = m.header.id().clone().into();
                    let consumer = self.net_engine_table.get(&id).unwrap();
                    consumer.consume(m);
                    // info!("net engine packet");
                }
                _ => {
                    // todo:
                    // handle through given routing_table
                }
            }
        }
    }
}