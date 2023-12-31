use tokio::net::TcpListener;

use async_trait::async_trait;

#[async_trait]
pub trait ServerSocketRepository {
    async fn bind_socket(&mut self, address: &str) -> Result<(), Box<dyn std::error::Error>>;
    fn get_listener(&self) -> Option<&TcpListener>;
    // async fn accept_client(&self);
}