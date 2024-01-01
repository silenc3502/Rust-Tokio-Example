use std::collections::HashMap;
use tokio::net::TcpListener;
use std::sync::{Arc, Mutex, Once};
use async_trait::async_trait;
use lazy_static::lazy_static;
use crate::client_socket_accept::entity::client_socket::ClientSocket;
use crate::client_socket_accept::repository::client_socket_accept_repository::ClientSocketAcceptRepository;

#[derive(Clone)]
pub struct ClientSocketAcceptRepositoryImpl {
    client_list: Arc<Mutex<HashMap<String, ClientSocket>>>,
}

impl ClientSocketAcceptRepositoryImpl {
    pub fn new() -> Self {
        ClientSocketAcceptRepositoryImpl {
            client_list: Arc::new(Mutex::new(HashMap::new())),
        }
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
                    self.client_list.lock().unwrap().insert(client.address().to_string(), client.clone());
                }
                Err(err) => {
                    // Handle the error (e.g., log or propagate)
                    eprintln!("Error accepting client: {:?}", err);
                }
            }
        }
    }
}

lazy_static! {
    static ref CLIENT_SOCKET_ACCEPT_REPOSITORY: Mutex<Option<Arc<ClientSocketAcceptRepositoryImpl>>> = Mutex::new(None);
    static ref INIT: Once = Once::new();
}

impl ClientSocketAcceptRepositoryImpl {
    pub fn get_instance() -> Arc<ClientSocketAcceptRepositoryImpl> {
        INIT.call_once(|| {
            let repository = ClientSocketAcceptRepositoryImpl::new();
            *CLIENT_SOCKET_ACCEPT_REPOSITORY.lock().unwrap() = Some(Arc::new(repository));
        });

        CLIENT_SOCKET_ACCEPT_REPOSITORY
            .lock()
            .unwrap()
            .as_ref()
            .unwrap()
            .clone()
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

        // Get the instance of ClientSocketAcceptRepository
        let repository = ClientSocketAcceptRepositoryImpl::get_instance();

        // Clone the Arc, not the TcpListener itself
        let listener_clone = listener.clone();

        // Spawn the accept_client function in a separate task
        tokio::spawn(async move {
            repository.accept_client(&listener_clone).await;
        });

        // Wait for a short duration to simulate client connections
        tokio::time::sleep(Duration::from_millis(100)).await;

        // Here, you can add assertions or checks based on your requirements
        // For example, you can check if the client_list has been updated with new clients.

        // In this example, we're just checking if the listener is still open
        assert!(listener.local_addr().is_ok());
    }
}
