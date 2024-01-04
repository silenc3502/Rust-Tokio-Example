use async_trait::async_trait;
use tokio::net::TcpStream;

#[async_trait]
pub trait ServerReceiverRepository {
    async fn receive(&mut self, stream: TcpStream);
}