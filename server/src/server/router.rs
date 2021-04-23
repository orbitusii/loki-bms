use loki_bms_common::protocol::client;
use crate::server::routes;

pub(crate) async fn route(msg: client::Message) {
    match msg {
        client::Message::Auth(msg) => routes::auth(msg).await
    }
}
