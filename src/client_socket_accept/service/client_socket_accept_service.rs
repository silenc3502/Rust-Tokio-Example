use std::sync::Arc;
use async_trait::async_trait;
use crate::utility::initializer::AcceptorChannel;

#[async_trait]
pub trait ClientSocketAcceptService: Send + Sync {
    async fn accept_client(&self);
    async fn inject_accept_channel(&self, acceptor_channel_arc: Arc<AcceptorChannel>);
}