use std::sync::Arc;
use tokio::net::TcpListener;

use async_trait::async_trait;
use crate::utility::initializer::AcceptorChannel;

#[async_trait]
pub trait ClientSocketAcceptRepository {
    async fn accept_client(&mut self, listener: &TcpListener);
    async fn inject_accept_channel(&mut self, acceptor_channel_arc: Arc<AcceptorChannel>);
}