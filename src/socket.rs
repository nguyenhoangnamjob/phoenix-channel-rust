use futures_util::StreamExt;
use futures_util::stream::SplitStream;
use send::Send;
use std::error::Error;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};
use tokio_tungstenite::{connect_async, tungstenite};
use url::Url;
use std::str;
use tokio_tungstenite::{ MaybeTlsStream, WebSocketStream};
use tokio::net::TcpStream;

use crate::{channel::Channel, send};

pub struct Socket {
    end_point: String,
    send: Option<Arc<Mutex<Send>>>,
    read: Option<SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>>,
    on_open_callback: Box<dyn Fn()>,
    on_close_callback: Box<dyn Fn()>,
    on_error_callback: Box<dyn Fn(Box<dyn Error>)>,
}

impl Socket {
    pub fn new(end_point: String) -> Result<Self, Box<dyn Error>> {
        Ok(Socket {
            end_point,
            send: None,
            read: None,
            on_open_callback: Box::new(|| {}),
            on_close_callback: Box::new(|| {}),
            on_error_callback: Box::new(|e: Box<dyn Error>| {}),
        })
    }

    pub fn end_point_url(&self) -> Url {
        let url = format!("{}/websocket", self.end_point);
        Url::parse(&url).unwrap()
    }

    pub fn on_open<F>(&mut self, callback: F)
    where
        F: Fn() + 'static,
    {
        self.on_open_callback = Box::new(callback)
    }

    pub fn on_close<F>(&mut self, callback: F)
    where
        F: Fn() + 'static,
    {
        self.on_close_callback = Box::new(callback)
    }

    pub fn on_error<F>(&mut self, callback: F)
    where
        F: Fn(Box<dyn Error>) + 'static,
    {
        self.on_error_callback = Box::new(callback)
    }

    fn send_heart_beat(&self, send: Arc<Mutex<Send>>) {
        tokio::spawn(async move {
            loop {
                {
                    let send = &mut send.lock().await;
                    send.send_heartbeat().await;
                }

                sleep(Duration::from_secs(30)).await;
            }
        });
    }

    pub async fn subscribe(&self, read: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>) {
        tokio::spawn(async move {
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
        });
    }

    pub async fn connect(&mut self) {
        let url = self.end_point_url();
        let url = match url.scheme() {
            "ws" | "wss" => url,

            _ => {
                let callback = &self.on_error_callback;
                callback(Box::new(tungstenite::error::UrlError::UnsupportedUrlScheme));

                panic!()
            }
        };

        let (ws_stream, _response) = match connect_async(url).await {
            Ok(w) => {
                let callback = &self.on_open_callback;
                callback();

                w
            }

            Err(e) => match e {
                tungstenite::Error::ConnectionClosed | tungstenite::Error::AlreadyClosed => {
                    let callback = &self.on_close_callback;
                    callback();

                    panic!()
                }

                _ => {
                    let callback = &self.on_error_callback;
                    callback(Box::new(e));

                    panic!()
                }
            },
        };

        let (write, read) = ws_stream.split();
        let send = Arc::new(Mutex::new(Send { write }));

        self.send_heart_beat(Arc::clone(&send));
        self.subscribe(read);

        self.send = Some(send);
    }

    pub fn channel(&self, topic: String) -> Channel {
        let send = Arc::clone(self.send.as_ref().unwrap());

        Channel {
            send,
            topic,
        }
    }
}
