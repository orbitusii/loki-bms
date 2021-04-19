use std::sync::Arc;

use crate::settings::Settings;

#[cfg(feature = "dcs")]
pub mod dcs;

pub async fn handle(settings: Arc<Settings>) {
    #[cfg(feature = "dcs")]
    dcs::handle(settings).await;
}
