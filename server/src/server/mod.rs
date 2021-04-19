use std::sync::Arc;

use log::{error, info, warn};

use crate::settings::Settings;
use tokio::net::TcpListener;

pub mod connection;

pub async fn start(settings: Arc<Settings>) {
    info!("Starting server...");

    let listener = match TcpListener::bind(settings.listen).await {
        Ok(l) => l,
        Err(e) => {
            error!("Unable to listen on {} because of: {}", settings.listen, e);
            panic!() // TODO: make the server quit because of this
        }
    };

    info!("Listening on {}", settings.listen);

    loop {
        let (socket, addr) = match listener.accept().await {
            Ok(socket) => socket,
            Err(e) => {
                warn!("Unable to open socket: {}", e);
                continue;
            }
        };
        tokio::spawn(connection::handle(socket, addr));
    }
}
