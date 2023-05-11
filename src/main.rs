use net::client::NetClient;


#[tokio::main]
async fn main() {
    env_logger::builder()
        .init();

    let s = NetClient {};
    let _ = s.start().await;
    println!("?");
}