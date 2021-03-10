use std::collections::HashMap;

#[derive(Debug, Extract, Deserialize, Serialize, Clone)]
pub struct Notification {
    pub channels: Vec<Channel>,
    pub message: Message,
}

#[derive(Debug, Extract, Deserialize, Serialize, Clone)]
pub struct Channel {
    pub channel: String,
    pub target: String,
    pub settings: HashMap<String, bool>,
}

#[derive(Debug, Extract, Deserialize, Serialize, Clone)]
pub struct Settings {
    pub all: bool,
    pub incoming: bool,
    pub outgoing: bool,
    pub news: bool,
    pub others: bool,
}

#[derive(Debug, Extract, Deserialize, Serialize, Clone)]
pub struct Message {
    pub title: I18N,
    pub body: I18N,
}

#[derive(Debug, Extract, Deserialize, Serialize, Clone)]
pub struct I18N {
    pub en: String,
    pub ro: String,
    pub ru: String,
}
