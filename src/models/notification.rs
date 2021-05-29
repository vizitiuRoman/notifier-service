#[derive(Debug, Extract, Deserialize, Serialize, Clone)]
pub struct Notification {
    pub login: String,
    pub channels: Vec<Channel>,
    pub title: String,
    pub body: String,
}

#[derive(Debug, Extract, Deserialize, Serialize, Clone)]
pub struct Channel {
    pub channel: String,
    pub target: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Message {
    pub title: String,
    pub body: String,
}
