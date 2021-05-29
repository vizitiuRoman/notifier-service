use crate::{
    external::notifier::{model::NotifierResponse, notifier::Notifiers},
    models::notification::{Channel, Message, Notification},
    services::services::NotificationSrv,
};

use log::{error, info};
use rayon::prelude::*;
use std::sync::Arc;

pub struct NotificationService {
    notifiers: Notifiers,
}

impl NotificationService {
    pub fn new(notifiers: Notifiers) -> Self {
        Self { notifiers }
    }
}

impl NotificationSrv for NotificationService {
    fn push_notifications(&self, notification: Arc<Notification>) -> Vec<NotifierResponse> {
        let notifier_response: Vec<NotifierResponse> = notification
            .channels
            .par_iter()
            .map(|channel: &Channel| {
                let channel_string = channel.channel.to_string();

                let notifier = match self.notifiers.get(&channel.channel) {
                    Some(notifier) => notifier,
                    None => {
                        return NotifierResponse::new(
                            Option::from(String::from("Not found channel")),
                            channel_string,
                        );
                    }
                };

                match notifier.send(
                    &channel.target,
                    Message { title: notification.title.clone(), body: notification.body.clone() },
                ) {
                    Ok(_) => {
                        info!(
                            "[push_notifications] to user: {}, channel: {}",
                            notification.login, channel_string
                        );
                        NotifierResponse::new(None, channel_string)
                    }
                    Err(err) => {
                        error!(
                            "[push_notifications] failed: {}, channel: {}, error: {:?}",
                            notification.login, channel_string, err.error
                        );
                        NotifierResponse::new(Option::from(err.error), channel_string)
                    }
                }
            })
            .collect();

        notifier_response
    }
}
