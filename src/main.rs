#[macro_use]
extern crate tower_web;
extern crate tokio;

use crate::external::notifier;

mod delivery;
mod external;
mod services;
mod models;

use tower_web::ServiceBuilder;
use delivery::http;
use std::collections::HashMap;
use dotenv::dotenv;

fn main() {
    dotenv().ok();

    let port = std::env::var("PORT").unwrap();
    let firebase_key = std::env::var("FIREBASEKEY").unwrap();

    let addr = format!("0.0.0.0:{}", port).parse().expect("Invalid address");
    println!("Listening on http://{}", addr);

    let mut notifiers: notifier::notifier::Notifiers = HashMap::new();
    notifiers.insert(
        String::from("application"),
        Box::new(notifier::firebase::Firebase::new(firebase_key)),
    );

    let manager = services::manager::Manager::new(
        notifiers,
    );

    ServiceBuilder::new()
        .resource(http::controller::Controller::new(manager))
        .run(&addr)
        .unwrap();
}
