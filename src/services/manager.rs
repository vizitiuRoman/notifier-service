use crate::{
    external::notifier::notifier::Notifiers, services::notification_service::NotificationService,
};

// Manager is just a collection of all services we have in the project
pub struct Manager {
    pub notification_service: NotificationService,
}

impl Manager {
    pub fn new(notifiers: Notifiers) -> Self {
        Self { notification_service: NotificationService::new(notifiers) }
    }
}
