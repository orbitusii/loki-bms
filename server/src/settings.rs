use std::net::SocketAddr;

use config as rsconfig;
use config::{ConfigError, File};

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub refresh_rate: u8,
    pub dcs_rpc: SocketAddr,
    pub listen: SocketAddr,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut config = rsconfig::Config::new();

        config.merge(File::with_name("default"))?;
        config.merge(File::with_name("settings").required(false))?;

        config.try_into()
    }
}
