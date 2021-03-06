use crate::{
    external::notifier::{
        model::{Error, FirebaseNotificationReq, FirebaseResponse},
        notifier::Notifier,
    },
    models::notification::Message,
};

use curl::easy::{Easy, List};
use std::io::Read;

pub struct Firebase {
    key: String,
}

impl Firebase {
    pub fn new(key: String) -> Self {
        Self { key }
    }
}

impl Notifier for Firebase {
    fn send(&self, target: &String, message: Message) -> Result<(), Error> {
        let firebase_notification_req =
            FirebaseNotificationReq { to: target.to_string(), notification: message };

        let fb_notification_req_str = serde_json::to_string(&firebase_notification_req).unwrap();

        let mut serialized = fb_notification_req_str.as_bytes();

        let mut easy = Easy::new();
        easy.url("https://fcm.googleapis.com/fcm/send").unwrap();
        easy.post(true).unwrap();
        easy.post_field_size(serialized.len() as u64).unwrap();

        let mut res: FirebaseResponse = FirebaseResponse {
            multicast_id: 0.0,
            success: 0,
            failure: 0,
            canonical_ids: 0,
            results: vec![],
        };

        let mut list = List::new();
        list.append(&format!("Authorization: key={}", self.key)).unwrap();
        list.append("Content-Type: application/json").unwrap();
        easy.http_headers(list).unwrap();

        {
            let mut transfer = easy.transfer();
            transfer.read_function(|buf| Ok(serialized.read(buf).unwrap_or(0))).unwrap();

            transfer
                .write_function(|data| {
                    let res_str = String::from_utf8(Vec::from(data)).unwrap();
                    match serde_json::from_str(&res_str) {
                        Ok::<FirebaseResponse, _>(firebase_response) => res = firebase_response,
                        _ => {}
                    };
                    Ok(data.len())
                })
                .unwrap();

            match transfer.perform() {
                Err(err) => return Err(Error { error: err.to_string() }),
                _ => {}
            }
        }

        if res.success == 0 {
            return Err(Error {
                error: res
                    .results
                    .iter()
                    .map(|v| match &v.error {
                        Some(err) => err.to_string(),
                        None => String::from("Unknown error"),
                    })
                    .collect(),
            });
        }

        Ok(())
    }
}
