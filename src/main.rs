use socket::Socket;
use std::collections::HashMap;

mod channel;
mod constants;
mod message;
mod reply;
mod send;
mod socket;

#[tokio::main]
async fn main() {
    let mut socket = Socket::new("ws://localhost:4000/socket".to_string()).unwrap();
    socket.connect().await;

    let mut channel = socket.channel("message".to_string());
    channel.join().await;

    let mut payload = HashMap::new();
    payload.insert("data".to_string(), "Hello world".to_string());

    channel.push("new_msg".to_string(), payload).await;

    loop {}
}
