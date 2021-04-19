use std::{sync::Arc, time::Duration};

use dcsjsonrpc_client::Client;
use log::{error, info};
use tokio::time::sleep;

use crate::settings::Settings;

const RETRY: Duration = Duration::from_secs(10);

pub async fn handle(settings: Arc<Settings>) {
    info!("Starting DCS track data handler...");

    let client = loop {
        match Client::<usize>::connect(settings.dcs_rpc) {
            Ok(c) => break c,
            Err(err) => {
                error!(
                    "Could not connect to DCS RPC due to: {:?}... Retrying in {:?}",
                    err, RETRY
                );
                sleep(RETRY).await;
            }
        }
    };

    loop {}
}
