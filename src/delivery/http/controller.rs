use crate::external::notifier::model::NotifierResponse;
use crate::services::services::NotificationSrv;
use crate::models::notification::Notification;
use crate::services::manager::Manager;

pub struct Controller {
    services: Manager
}

impl Controller {
    pub fn new(services: Manager) -> Self {
        Controller {
            services
        }
    }
}

impl_web! {
    impl Controller {
        #[get("/ping")]
        fn ping(&self) -> Result<&'static str, ()> {
            Ok("Ping")
        }

        #[post("/api/v1/notifications")]
        #[content_type("json")]
        fn push_notification(&self, body: Notification) -> Result<Vec<NotifierResponse>, ()> {
            let notitication = std::sync::Arc::new(body);
            let notifier_response = self.services.notification_service.push_notifications(notitication);
           Ok(notifier_response)
        }
    }
}
