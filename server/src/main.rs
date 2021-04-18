#![feature(never_type)]

extern crate serde;

#[macro_use]
extern crate serde_derive;

use std::net::IpAddr;
use clap::Clap;
use dcsjsonrpc_client::Client;
use crate::settings::Settings;
use tokio::net::TcpListener;
use std::error::Error;
use tokio::sync::Mutex;
use std::time::Duration;
use std::sync::Arc;
use tokio::io::AsyncReadExt;

mod trackfile;
mod settings;

#[derive(Clap)]
struct Opts {
    #[clap(long)]
    listen_ip: IpAddr,
    #[clap(long)]
    listen_port: u16,
    #[clap(long)]
    dcs_rpc_ip: IpAddr,
    #[clap(long)]
    dcs_rpc_port: u16,
}

#[tokio::main]
async fn main() -> Result<!, Box<dyn Error>> {
    let Opts { listen_ip, listen_port, dcs_rpc_ip, dcs_rpc_port } = Opts::parse();
    let settings = Settings::new().unwrap();

    let dcs_client = Client::<usize>::connect(format!("{}:{}", dcs_rpc_ip, dcs_rpc_port)).unwrap();

    let listener = TcpListener::bind(format!("{}:{}", listen_ip, listen_port)).await?;

    // let trackfiles = Arc::new(Mutex::new(Vec::new()));

    tokio::spawn(async move {
        // let mut trackfiles = trackfiles.lock().await;


        tokio::time::sleep(Duration::from_secs(settings.refresh_rate as u64));
    });

    loop {
        let (mut socket, _) = listener.accept().await?;

    }
}
