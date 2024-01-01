use async_trait::async_trait;

#[async_trait]
pub trait ClientSocketAcceptController: Send + Sync {
    async fn accept_client(&self);
}