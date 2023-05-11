use log::{info};
use tokio::io::AsyncReadExt;
use tokio::runtime::Builder;
use tokio::net::{TcpListener, TcpStream};
use crate::connection::NetConnection;

pub struct NetServer {}

impl NetServer {
    pub fn run(&self) {
        let acceptor_runtime = Builder::new_multi_thread()
            .worker_threads(1)
            .thread_name("acceptor-pool")
            .thread_stack_size(3 * 1024 * 1024)
            .enable_time()
            .enable_io()
            .build()
            .unwrap();

        let request_runtime = Builder::new_multi_thread()
            .worker_threads(2)
            .thread_name("request-pool")
            .thread_stack_size(3 * 1024 * 1024)
            .enable_time()
            .enable_io()
            .build()
            .unwrap();

        acceptor_runtime.block_on(async {
            let listener = TcpListener::bind("127.0.0.1:15779").await.unwrap();
            loop {
                let (socket, _) = listener.accept().await.unwrap();
                info!("✔ connection established");
                let _g = request_runtime.enter();
                let session = NetConnection::new(socket);
                request_runtime.spawn(session.recv_o());
            }
        });
    }
}
