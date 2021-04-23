#![feature(never_type)]
#![feature(assert_matches)]

extern crate serde;

#[macro_use]
extern crate log;

#[macro_use]
extern crate serde_derive;

use crate::settings::Settings;
use std::sync::Arc;
use std::error::Error;

mod server;
mod settings;
mod tracks;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let settings = Arc::new(Settings::new().unwrap());

    // If either task returns an error or panics, the application should quit
    futures::try_join!(
        tokio::spawn(server::start(settings.clone())),
        tokio::spawn(tracks::handle(settings.clone())),
    )?;

    Ok(())
}
