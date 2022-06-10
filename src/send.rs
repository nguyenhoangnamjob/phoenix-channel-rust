use constants::{
    CHANNEL_EVENTS_CLOSE, CHANNEL_EVENTS_ERROR, CHANNEL_EVENTS_JOIN, CHANNEL_EVENTS_LEAVE,
    CHANNEL_EVENTS_REPLY,
};
use futures_util::{stream::SplitSink, SinkExt};
use message::{ChannelEvents, MessageJson};
use serde_json::json;
use std::{collections::HashMap, str};
use tokio::net::TcpStream;
use tokio_tungstenite::{tungstenite::protocol::Message, MaybeTlsStream, WebSocketStream};

use crate::{constants, message};

pub struct Send {
    pub write: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
}

impl Send {
    pub async fn send_message(&mut self, message_json: MessageJson) {
        let message = json!(message_json);

        self.write
            .send(Message::Text(message.to_string()))
            .await
            .unwrap();
    }

    pub async fn send_channel_event(
        &mut self,
        topic: String,
        event: ChannelEvents,
        payload: HashMap<String, String>,
    ) {
        let event: &str = match event {
            ChannelEvents::Close => CHANNEL_EVENTS_CLOSE,
            ChannelEvents::Error => CHANNEL_EVENTS_ERROR,
            ChannelEvents::Reply => CHANNEL_EVENTS_REPLY,
            ChannelEvents::Join => CHANNEL_EVENTS_JOIN,
            ChannelEvents::Leave => CHANNEL_EVENTS_LEAVE,
        };

        let message = MessageJson {
            topic,
            event: event.to_owned(),
            payload,
            refer: "0".to_owned(),
        };

        self.send_message(message).await;
    }

    pub async fn send_heartbeat(&mut self) {
        let message = MessageJson {
            topic: "phoenix".to_owned(),
            event: "heartbeat".to_owned(),
            payload: HashMap::new(),
            refer: "0".to_owned(),
        };

        self.send_message(message).await;
    }
}
