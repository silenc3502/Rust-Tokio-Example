use async_trait::async_trait;

#[async_trait]
pub trait ServerSocketService {
    async fn bind(&mut self, address: &str) -> Result<(), Box<dyn std::error::Error>>;
}