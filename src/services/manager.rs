use crate::services::notification_service::NotificationService;
use crate::external::notifier::notifier::Notifiers;

// Manager is just a collection of all services we have in the project
pub struct Manager {
    pub notification_service: NotificationService
}

impl Manager {
    pub fn new(notifiers: Notifiers) -> Self {
        Manager {
            notification_service: NotificationService::new(
                notifiers
            )
        }
    }
}
