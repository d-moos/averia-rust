use std::mem::transmute_copy;
use tokio::join;
use tokio::net::{TcpStream};
use tokio::spawn;
use protocol::message::message::Message;
use crate::connection::NetConnection;

pub struct NetClient {}

impl NetClient {
    pub async fn start(&self) -> Result<(), std::io::Error> {
        let mut stream = TcpStream::connect("filter.evolin.net:4001").await?;
        // let connection = NetConnection::new(stream);

        let (proc, recv) = NetConnection::create(&mut stream);

        join!(proc.run(), recv.recv());

        // spawn(&mut connection.proc_inbound());

        // connection.recv().await;

        // tokio::spawn(connection.recv());
        // tokio::spawn(connection.proc_inbound());

        Ok(())
    }
}