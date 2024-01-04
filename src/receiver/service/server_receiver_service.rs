use async_trait::async_trait;

#[async_trait]
pub trait ServerReceiverService {
    async fn client_receive(&mut self);
}