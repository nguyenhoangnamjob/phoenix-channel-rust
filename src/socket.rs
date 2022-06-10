use futures_util::StreamExt;
use send::Send;
use std::{error::Error, str};
use tokio_tungstenite::connect_async;
use url::Url;

use crate::{channel::Channel, send};

pub struct Socket {
    end_point: String,
    send: Option<Send>,
}

impl Socket {
    pub fn new(end_point: String) -> Result<Self, Box<dyn Error>> {
        Ok(Socket {
            end_point,
            send: None,
        })
    }

    pub fn end_point_url(&self) -> Url {
        let url = format!("{}/websocket", self.end_point);
        Url::parse(&url).unwrap()
    }

    // Validate end point
    pub async fn connect(&mut self) {
        let (ws_stream, _response) = connect_async(self.end_point_url())
            .await
            .expect("Can't connect");

        let (write, read) = ws_stream.split();

        let mut send = Send { write };
        send.send_heartbeat().await;

        self.send = Some(send);

        let ws_to_stdout = {
            read.for_each(|message| async {
                let data = message.unwrap().into_data();
                match str::from_utf8(&data) {
                    Ok(v) => println!("{}", v),
                    Err(e) => panic!("Invalid UTF-8 {}", e),
                }
            })
        };

        ws_to_stdout.await;
    }

    pub fn channel(&mut self, topic: String) -> Channel {
        Channel {
            send: self.send,
            topic,
        }
    }
}
