use std::collections::HashMap;
use tokio::net::TcpListener;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex as AsyncMutex;
use async_trait::async_trait;
use lazy_static::lazy_static;
use crate::client_socket_accept::entity::client_socket::ClientSocket;
use crate::client_socket_accept::repository::client_socket_accept_repository::ClientSocketAcceptRepository;

#[derive(Clone)]
pub struct ClientSocketAcceptRepositoryImpl {
    client_list: Arc<AsyncMutex<HashMap<String, ClientSocket>>>,
}

impl ClientSocketAcceptRepositoryImpl {
    pub fn new() -> Self {
        ClientSocketAcceptRepositoryImpl {
            client_list: Arc::new(AsyncMutex::new(HashMap::new())),
        }
    }

    pub fn get_instance() -> Arc<AsyncMutex<ClientSocketAcceptRepositoryImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<ClientSocketAcceptRepositoryImpl>> =
                Arc::new(AsyncMutex::new(ClientSocketAcceptRepositoryImpl::new()));
        }
        INSTANCE.clone()
    }

    pub fn get_client_list(&self) -> &Arc<AsyncMutex<HashMap<String, ClientSocket>>> {
        &self.client_list
    }
}

#[async_trait]
impl ClientSocketAcceptRepository for ClientSocketAcceptRepositoryImpl {

    async fn accept_client(&self, listener: &TcpListener) {
        println!("Client Socket Accept Repository: accept()");
        loop {
            match listener.accept().await {
                Ok((stream, peer_addr)) => {
                    println!("Accepted client from: {}", peer_addr);
                    // Create a new SocketClient for the accepted client
                    let client = ClientSocket::new(peer_addr.to_string(), stream);

                    // Add the client to the client list
                    let mut client_list_gaurd = self.client_list.lock().await;
                    client_list_gaurd.insert(client.address().to_string(), client.clone());
                }
                Err(err) => {
                    // Handle the error (e.g., log or propagate)
                    eprintln!("Error accepting client: {:?}", err);
                }
            }

            tokio::time::sleep(Duration::from_millis(300)).await;
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;
    use super::*;

    #[tokio::test]
    async fn test_singleton_behavior() {
        let instance1 = ClientSocketAcceptRepositoryImpl::get_instance();
        let instance2 = ClientSocketAcceptRepositoryImpl::get_instance();

        assert!(Arc::ptr_eq(&instance1, &instance2));
    }

    #[tokio::test]
    async fn test_accept_client() {
        let listener = Arc::new(TcpListener::bind("127.0.0.1:7128").await.expect("Failed to bind to address"));
        let repository = ClientSocketAcceptRepositoryImpl::get_instance();
        let listener_clone = listener.clone();

        let thread_object = tokio::spawn(async move {
            let repository_gaurd = repository.lock().await;
            repository_gaurd.accept_client(&listener_clone).await;
        });

        tokio::time::sleep(Duration::from_secs(5)).await;

        assert!(listener.local_addr().is_ok());
    }
}
