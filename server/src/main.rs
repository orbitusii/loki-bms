#![feature(never_type)]

extern crate serde;

#[macro_use]
extern crate log;

#[macro_use]
extern crate serde_derive;

use crate::settings::Settings;
use std::sync::Arc;

mod server;
mod settings;
mod tracks;

#[tokio::main]
async fn main() {
    env_logger::init();

    let settings = Arc::new(Settings::new().unwrap());

    // If either task returns an error or panics, the application should quit
    futures::try_join!(
        tokio::spawn(server::start(settings.clone())),
        tokio::spawn(tracks::handle(settings.clone())),
    );
}
