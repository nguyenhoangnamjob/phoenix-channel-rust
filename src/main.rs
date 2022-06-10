use socket::Socket;
use std::{collections::HashMap, io::stdin};

mod channel;
mod constants;
mod message;
mod send;
mod socket;

#[tokio::main]
async fn main() {
    let mut socket = Socket::new("ws://localhost:4000/socket".to_string()).unwrap();
    socket.connect().await;

    let mut channel = socket.channel("message".to_string());
    channel.join();

    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();

    let mut payload = HashMap::new();
    payload.insert("data".to_owned(), line);

    channel.push("message:new".to_string(), payload);
}
