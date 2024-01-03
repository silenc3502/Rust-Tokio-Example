use async_trait::async_trait;

#[async_trait]
pub trait ServerSocketService {
    async fn server_socket_bind(&mut self, address: &str) -> Result<(), Box<dyn std::error::Error>>;
}