use tokio::net::TcpListener;

use async_trait::async_trait;

#[async_trait]
pub trait ClientSocketRepository {
    async fn accept_client(&self);
}