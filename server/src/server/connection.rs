use std::net::SocketAddr;
use tokio::net::TcpStream;

pub async fn handle(mut socket: TcpStream, addr: SocketAddr) {
    info!("Client {} connected on port {}", addr.ip(), addr.port());
}
