use tokio::sync::mpsc::Sender;
use log::{info, trace};
use tokio::net::TcpStream;
use tokio::io::AsyncReadExt;
use protocol::message::header;
use protocol::message::header::{Header, MAX_HEADER_SIZE};
use protocol::message::message::Message;

pub struct NetReceiver<'a> {
    tx: Sender<Message>,
    incomplete_ptr: usize,
    incomplete_buffer: [u8; 4096],

    stream: &'a mut TcpStream
}

impl<'a> NetReceiver<'a> {
    pub fn new(tx: Sender<Message>, stream: &'a mut TcpStream) -> Self {
        NetReceiver {
            tx,
            incomplete_ptr: 0,
            incomplete_buffer: [0; 4096],
            stream
        }
    }

    pub async fn recv(mut self) {
        let mut buffer = [0; 8192];
        loop {
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

                    // todo: error handling when channel send does not succeed.
                    let _ = self.tx.send(message).await;
                } else {
                    // worst-case; the inbound message did not contain enough data to finish the message
                    self.incomplete_ptr += size_to_copy;
                }
            }


            // there's at least one full message header in the stream which we can process
            while len - ptr >= MAX_HEADER_SIZE as usize {
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

                // todo: error handling when channel send does not succeed.
                let _ = self.tx.send(message).await;

                ptr += message_size as usize;
            }
        }
    }
}