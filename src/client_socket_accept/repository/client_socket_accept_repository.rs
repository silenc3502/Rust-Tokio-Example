use tokio::net::TcpListener;

use async_trait::async_trait;

#[async_trait]
pub trait ClientSocketAcceptRepository {
    async fn accept_client(&self, listener: &TcpListener);
}