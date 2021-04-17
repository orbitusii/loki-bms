mod coordinates;
mod scope;

use iced::{
    Sandbox, window
};
use scope::MainScope;

fn main() {
    println!("Starting LOKI Battle Management System...");

    let window = window::Settings {
        size: (1024, 768),
        resizable: true,
        min_size: Some((700, 400)),
        ..window::Settings::default()
    };

    let settings = iced::Settings {
        window,
        ..iced::Settings::default()
    };

    MainScope::run(settings).unwrap();

    println!("Exiting...");
}