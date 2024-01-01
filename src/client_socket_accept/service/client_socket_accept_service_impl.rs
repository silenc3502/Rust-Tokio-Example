use std::sync::Arc;
use std::time::Duration;
use async_trait::async_trait;
use lazy_static::lazy_static;
use crate::client_socket_accept::repository::client_socket_accept_repository::ClientSocketAcceptRepository;
use crate::client_socket_accept::repository::client_socket_accept_repository_impl::ClientSocketAcceptRepositoryImpl;
use crate::client_socket_accept::service::client_socket_accept_service::ClientSocketAcceptService;
use crate::server_socket::repository::ServerSocketRepository::ServerSocketRepository;
use crate::server_socket::repository::ServerSocketRepositoryImpl::ServerSocketRepositoryImpl;
use crate::server_socket::service::ServerSocketService::ServerSocketService;

#[derive(Clone)]
pub struct ClientSocketAcceptServiceImpl {
    client_socket_accept_repository: ClientSocketAcceptRepositoryImpl,
    server_socket_repository: ServerSocketRepositoryImpl
}

impl ClientSocketAcceptServiceImpl {
    pub fn new(client_socket_accept_repository: ClientSocketAcceptRepositoryImpl,
               server_socket_repository: ServerSocketRepositoryImpl) -> Self {

        ClientSocketAcceptServiceImpl {
            client_socket_accept_repository,
            server_socket_repository
        }
    }
}

lazy_static! {
    static ref CLIENT_SOCKET_ACCEPT_SERVICE: Arc<ClientSocketAcceptServiceImpl> = {
        Arc::new(ClientSocketAcceptServiceImpl::new(
            ClientSocketAcceptRepositoryImpl::get_instance().as_ref().clone(),
            ServerSocketRepositoryImpl::new(),
        ))
    };
}

impl ClientSocketAcceptServiceImpl {
    pub fn get_instance() -> Arc<ClientSocketAcceptServiceImpl> {
        CLIENT_SOCKET_ACCEPT_SERVICE.clone()
    }
}

#[async_trait]
impl ClientSocketAcceptService for ClientSocketAcceptServiceImpl {

    async fn accept_client(&self) {
        println!("Client Socket Accept Service: accept()");

        if let Some(listener) = self.server_socket_repository.get_listener() {
            self.client_socket_accept_repository.accept_client(listener).await;
        } else {
            // Handle the case when the listener is not available
            eprintln!("Listener not available for accepting clients.");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::Duration;
    use tokio::net::TcpListener;

    #[tokio::test]
    async fn test_singleton_behavior() {
        // Set up your repository instance
        let instance1 = ClientSocketAcceptServiceImpl::get_instance();
        let instance2 = ClientSocketAcceptServiceImpl::get_instance();

        // Ensure that the instances are the same
        assert!(Arc::ptr_eq(&instance1, &instance2));
    }

    #[tokio::test]
    async fn test_accept_client() {
        // Set up your repository instance
        let repository = ClientSocketAcceptRepositoryImpl::get_instance();

        // Use Tokio's spawn to run the async method
        tokio::spawn(async move {
            // Create a dummy TcpListener for testing
            let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();

            // Call accept_client and pass the listener
            repository.accept_client(&listener).await;
        });

        // Sleep for a short duration to allow the async method to run
        tokio::time::sleep(Duration::from_secs(1)).await;

        // The test passes if it reaches this point without panicking
    }
}
