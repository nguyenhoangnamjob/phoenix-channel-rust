use std::collections::HashMap;

use send::Send;

use crate::{
    message::{ChannelEvents, MessageJson},
    send,
};

pub struct Channel {
    pub send: Option<Send>,
    pub topic: String,
}

impl Channel {
    pub fn join(&mut self) {
        self.send.unwrap().send_channel_event(
            self.topic.clone(),
            ChannelEvents::Join,
            HashMap::new(),
        );
    }

    pub fn push(&mut self, event: String, payload: HashMap<String, String>) {
        self.send.unwrap().send_message(MessageJson {
            topic: self.topic.clone(),
            event,
            payload,
            refer: "0".to_string(),
        });
    }
}
