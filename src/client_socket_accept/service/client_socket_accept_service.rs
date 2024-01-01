use async_trait::async_trait;

#[async_trait]
pub trait ClientSocketAcceptService: Send + Sync {
    async fn accept_client(&self);
}