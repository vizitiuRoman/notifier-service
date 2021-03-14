use crate::services::services::NotificationSrv;
use crate::models::notification::{Notification, Channel};
use crate::external::notifier::model::{NotifierResponse};
use crate::external::notifier::notifier::Notifiers;

use std::sync::Arc;

use rayon::prelude::*;

pub struct NotificationService {
    notifiers: Notifiers,
}

impl NotificationService {
    pub fn new(
        notifiers: Notifiers,
    ) -> Self {
        Self { notifiers }
    }
}

impl NotificationSrv for NotificationService {
    fn push_notifications(&self, notification: Arc<Notification>) -> Vec<NotifierResponse> {
        let notifier_response: Vec<NotifierResponse> = notification.channels.par_iter()
            .map(|channel: &Channel| {
                let notifier = match self.notifiers.get(&channel.channel) {
                    Some(notifier) => notifier,
                    None => return NotifierResponse::new(
                        Option::from(String::from("Not found channel")),
                        channel.channel.to_string(),
                    )
                };

                match notifier.send(&channel.target, &notification.message) {
                    Ok(_) => NotifierResponse::new(None, channel.channel.to_string()),
                    Err(err) => NotifierResponse::new(
                        Option::from(err.error),
                        channel.channel.to_string(),
                    )
                }
            })
            .collect();

        notifier_response
    }
}
