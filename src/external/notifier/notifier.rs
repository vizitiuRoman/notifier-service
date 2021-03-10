use crate::external::notifier::model::{Error};
use crate::models::notification::Message;

use std::collections::HashMap;

pub trait Notifier {
    fn send(&self, target: &String, message: &Message) -> Result<(), Error>;
}

pub type Notifiers = HashMap<String, Box<dyn Notifier + Send + Sync>>;
