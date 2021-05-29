#[macro_use]
extern crate tower_web;
extern crate tokio;

use crate::external::notifier;

mod delivery;
mod external;
mod models;
mod services;

use delivery::http;
use dotenv::dotenv;
use env_logger::Builder;
use log::{info, LevelFilter};
use std::collections::HashMap;
use tower_web::ServiceBuilder;

fn main() {
    dotenv().ok();

    let port = std::env::var("PORT").unwrap();
    let firebase_key = std::env::var("FIREBASEKEY").unwrap();

    let mut builder = Builder::from_default_env();
    builder.filter(None, LevelFilter::Info).init();

    let addr = format!("0.0.0.0:{}", port).parse().expect("Invalid address");
    info!("Listening on http://{}", addr);

    let mut notifiers: notifier::notifier::Notifiers = HashMap::new();
    notifiers.insert(
        String::from("application"),
        Box::new(notifier::firebase::Firebase::new(firebase_key)),
    );

    let manager = services::manager::Manager::new(notifiers);

    ServiceBuilder::new().resource(http::controller::Controller::new(manager)).run(&addr).unwrap();
}
