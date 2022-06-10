use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub enum ChannelEvents {
    Close,
    Error,
    Join,
    Reply,
    Leave,
}

#[derive(Deserialize, Serialize)]
pub struct MessageJson {
    pub topic: String,
    pub event: String,
    pub payload: HashMap<String, String>,

    #[serde(rename(serialize = "ref", deserialize = "ref"))]
    pub refer: String,
}
