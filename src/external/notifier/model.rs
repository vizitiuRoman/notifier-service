use crate::models::notification::Message;

#[derive(Serialize, Deserialize, Debug, Response)]
pub struct NotifierResponse {
    pub success: bool,
    pub error: Option<String>,
    pub channel: String,
}

impl NotifierResponse {
    pub fn new(error: Option<String>, channel: String) -> Self {
        NotifierResponse{
            success: error.is_none(),
            error,
            channel,
        }
    }
}

// FIREBASE

#[derive(Deserialize, Serialize)]
pub struct FirebaseNotificationReq {
    pub to: String,
    pub notification: Message,
    pub data: Message,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FirebaseResponse {
    pub multicast_id: f64,
    pub success: i64,
    pub failure: i64,
    pub canonical_ids: i64,
}

// ERROR

#[derive(Serialize, Deserialize, Debug)]
pub struct Error {
    pub error: String,
}
