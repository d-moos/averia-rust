use std::collections::HashMap;
use std::sync::Mutex;
use std::time::Duration;
use log::{info, trace};
use queues::{IsQueue, Queue};
use tokio::net::TcpStream;
use tokio::io::AsyncReadExt;
use tokio::sync::mpsc::{Receiver, Sender};
use protocol::message::header::{Header, MAX_HEADER_SIZE};
use protocol::message::{header, message};
use protocol::message::{message::Message, message_type::MessageType};
use crate::processor::NetProcessor;
use crate::receiver::NetReceiver;

pub struct NetConnection {
    stream: TcpStream,

    // indicates how much space is used in `incomplete_buffer`
    incomplete_ptr: usize,
    incomplete_buffer: [u8; 4096],

    inbound_queue: Mutex<Queue<Message>>,


    // receiver: NetReceiver<'a>,
    // processor: NetProcessor
}

impl NetConnection {
    pub fn new(stream: TcpStream) -> NetConnection {
        let (tx, rx) = tokio::sync::mpsc::channel::<Message>(1234);
        NetConnection {
            incomplete_ptr: 0,
            incomplete_buffer: [0; 4096],
            inbound_queue: Mutex::new(Queue::new()),
            stream,
        }
    }

    pub fn create(stream: &mut TcpStream) -> (NetProcessor, NetReceiver) {
        let (tx, rx) = tokio::sync::mpsc::channel::<Message>(1234);

        (NetProcessor::new(rx, HashMap::new()), NetReceiver::new(tx, stream))
    }

    pub async fn proc_inbound_o(&mut self) {
        loop {
            trace!("proc_inbound");
            let mut queue = self.inbound_queue.lock().unwrap();

            if queue.size() == 0 {
                tokio::time::sleep(Duration::from_millis(100)).await
            } else {
                let m = queue.remove().unwrap();
                info!("📥 {}", m);

                match m.header.id().message_type {
                    MessageType::NetEngine => {
                        info!("net engine packet");
                    }
                    _ => {
                        // todo:
                        // handle through given routing_table
                    }
                }
            }
        }
    }

    // RUNS IN ITS OWN THREAD
    pub async fn recv_o(mut self) {
        let mut buffer = [0; 8192];
        loop {
            trace!("recv");
            self.stream.readable().await.unwrap();
            let len = match self.stream.read(&mut buffer).await {
                Ok(n) => n,
                Err(e) => {
                    println!("failed to read from socket; err = {:?}", e);
                    return;
                }
            };

            if len == 0 {
                info!("❌ connection closed");
                return;
            }

            let mut ptr = 0;
            let mut queue = self.inbound_queue.lock().unwrap();

            if self.incomplete_ptr > 0 {
                // we have part of a message stored in our incomplete buffer
                // we must complete the data with the incoming data to build a valid message

                let incomplete_header = Header::from(&self.incomplete_buffer[ptr..MAX_HEADER_SIZE as usize]);
                let packet_size = (MAX_HEADER_SIZE + incomplete_header.data_size()) as usize;
                let missing_data = packet_size - self.incomplete_ptr;

                // worst-case is that the inbound buffer does not contain all of the remaining data
                let size_to_copy = if len > missing_data { missing_data } else { len };
                self.incomplete_buffer[self.incomplete_ptr..size_to_copy].copy_from_slice(&buffer[..size_to_copy]);

                if len > missing_data {
                    // we were able to copy all of the remaining data!
                    self.incomplete_ptr = 0;

                    // build (and decode) the message and add it into the queue
                    let message = Message::from(&self.incomplete_buffer[..packet_size]);
                    if message.is_encrypted() {
                        // todo: blowfish decode
                    }

                    let _ = queue.add(message);
                } else {
                    // worst-case; the inbound message did not contain enough data to finish the message
                    self.incomplete_ptr += size_to_copy;
                }
            }



            // there's at least one full message header in the stream which we can process
            while len - ptr >= header::MAX_HEADER_SIZE as usize {
                let header = Header::from(&buffer[ptr..MAX_HEADER_SIZE as usize]);
                let message_size = header.data_size();
                let recv_len = len - ptr;

                if recv_len < message_size as usize {
                    // message is not fully available in this stream
                    // put into buffer so that it can be completed with the next inbound
                    self.incomplete_buffer.copy_from_slice(&buffer[ptr..]);
                    self.incomplete_ptr = recv_len;
                    break;
                }


                let message_buffer = if header.is_encrypted() {
                    // todo: blowfish decode
                    &buffer[ptr..]
                } else {
                    &buffer[ptr..]
                };

                let message = Message::from(message_buffer);

                // todo: massive handling

                let _ = queue.add(message);

                ptr += message_size as usize;
            }
        }
    }
}