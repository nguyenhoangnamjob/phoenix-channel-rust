use std::{collections::HashMap, sync::Arc};

use send::Send;
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::{
    message::{ChannelEvents, MessageJson},
    send
};

pub struct Channel {
    pub send: Arc<Mutex<Send>>,
    pub topic: String,
}

impl Channel {
    pub async fn join(&mut self) {
        let send = &mut self.send.lock().await;

        send.send_channel_event(self.topic.clone(), ChannelEvents::Join, HashMap::new())
            .await;
    }

    pub async fn push(&mut self, event: String, payload: HashMap<String, String>) {
        let send = &mut self.send.lock().await;

        send.send_message(MessageJson {
            topic: self.topic.clone(),
            event,
            payload,
            refer: Uuid::new_v4().to_string(),
        })
        .await;
    }

    pub fn on<F>(&self, event: String, callback: F)
    where
        F: Fn(HashMap<String, String>),
    {
    }
}
