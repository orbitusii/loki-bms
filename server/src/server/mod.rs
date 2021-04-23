use std::sync::Arc;

use log::{error, info, warn};

use crate::settings::Settings;
use tokio::net::TcpListener;

pub mod connection;
mod router;
mod routes;

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

#[cfg(test)]
pub mod tests {
    use std::net::{Ipv4Addr, SocketAddr};

    use loki_bms_common::protocol::{client, server};
    use tokio::net::{TcpListener, TcpStream};
    use tokio_serde::formats::Bincode;
    use tokio_util::codec::{Framed, LengthDelimitedCodec};

    pub async fn testing_server() -> TcpListener {
        TcpListener::bind(SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 1338))
            .await
            .unwrap()
    }
}
