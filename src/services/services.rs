use crate::external::notifier::model::NotifierResponse;
use crate::models::notification::Notification;
use std::sync::Arc;

pub trait NotificationSrv {
    fn push_notifications(&self, notification: Arc<Notification>) -> Vec<NotifierResponse>;
}
