use futures::StreamExt;
use std::net::SocketAddr;
use tokio::net::TcpStream;
use tokio_serde::formats::Bincode;
use tokio_util::{
    codec::Framed,
    codec::LengthDelimitedCodec,
};
use crate::server::router;

pub fn stream_with_codec<I, S>(
    socket: TcpStream,
) -> tokio_serde::Framed<Framed<TcpStream, LengthDelimitedCodec>, I, S, Bincode<I, S>> {
    tokio_serde::Framed::new(
        Framed::new(socket, LengthDelimitedCodec::default()),
        Bincode::default(),
    )
}

pub async fn handle(socket: TcpStream, addr: SocketAddr) {
    let mut codec = stream_with_codec::<_, ()>(socket);

    info!("Client {} connected on port {}", addr.ip(), addr.port());

    for msg in codec.next().await {
        match msg {
            // Consider if we want to handle messages in parallel... this would add complexity...
            Ok(msg) => router::route(msg).await,
            Err(e) => {
                log::warn!("Socket with {}:{} closed because of {}", addr.ip(), addr.port(), e);
                return
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use client::AuthMessage;
    use futures::{join, SinkExt, StreamExt};
    use loki_bms_common::protocol::client;
    use serde::Serialize;
    use std::{
        io::Error,
        net::{Ipv4Addr, SocketAddr},
    };
    use tokio::net::{TcpListener, TcpStream};

    use super::stream_with_codec;

    async fn send_test_message<T>(msg: T) -> Result<(), Error>
    where
        T: Unpin + Serialize,
    {
        let client = TcpStream::connect(SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 1338))
            .await
            .unwrap();
        let mut stream = stream_with_codec::<client::Message, _>(client);
        stream.send(msg).await
    }

    async fn receive_test_message(server: TcpListener) -> client::Message {
        let (socket, _) = server.accept().await.unwrap();

        stream_with_codec::<_, ()>(socket)
            .next()
            .await
            .unwrap()
            .unwrap()
    }

    #[tokio::test]
    async fn deserializes_connect() {
        let server = crate::server::tests::testing_server().await;

        let message = client::Message::Auth(AuthMessage {
            username: None,
            password: None,
        });

        let (received_message, _) = join!(receive_test_message(server), send_test_message(message));
        if let client::Message::Auth(AuthMessage { username, password }) = received_message {
            assert_ne!(username, Some("hello".to_string()));
            assert_eq!(password, None);
        } else {
            panic!("unexpected")
        }
    }
}
