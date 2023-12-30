use tokio::net::TcpListener;
use std::sync::{Arc, Mutex, Once};
use async_trait::async_trait;
use lazy_static::lazy_static;
use crate::server_socket::repository::ServerSocketRepository::ServerSocketRepository;

pub struct ServerSocketRepositoryImpl {
    listener: Option<TcpListener>,
}

impl ServerSocketRepositoryImpl {
    pub fn new() -> Self {
        ServerSocketRepositoryImpl { listener: None }
    }
}

#[async_trait]
impl ServerSocketRepository for ServerSocketRepositoryImpl {
    async fn bind_socket(&mut self, address: &str) -> Result<(), Box<dyn std::error::Error>> {
        let listener = TcpListener::bind(address)
            .await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

        self.listener = Some(listener);
        Ok(())
    }

    fn get_listener(&self) -> Option<&tokio::net::TcpListener> {
        self.listener.as_ref()
    }
}

lazy_static! {
    static ref SERVER_SOCKET_REPOSITORY: Mutex<Option<ServerSocketRepositoryImpl>> = Mutex::new(None);
    static ref INIT: Once = Once::new();
}

fn get_server_socket_repository() -> &'static Mutex<Option<ServerSocketRepositoryImpl>> {
    INIT.call_once(|| {
        let repository = ServerSocketRepositoryImpl { listener: None };
        *SERVER_SOCKET_REPOSITORY.lock().unwrap() = Some(repository);
    });
    &SERVER_SOCKET_REPOSITORY
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::Duration;

    #[tokio::test]
    async fn test_bind_socket() {
        let repository = Arc::new(Mutex::new(ServerSocketRepositoryImpl::new()));
        let address = "127.0.0.1:7373";

        // Lock the mutex to get a mutable reference to ServerSocketRepositoryImpl
        let mut repository_guard = repository.lock().unwrap();

        // Call the bind_socket method on the inner ServerSocketRepositoryImpl
        match repository_guard.bind_socket(address).await {
            Ok(()) => {
                // The bind_socket method returned Ok
                // Additional test logic if needed...
            }
            Err(err) => {
                // The bind_socket method returned an error
                panic!("Error binding socket: {:?}", err);
            }
        }
    }

    #[tokio::test]
    async fn test_get_listener_unbound() {
        let repository = Arc::new(Mutex::new(ServerSocketRepositoryImpl::new()));

        // Lock the mutex to get a mutable reference to ServerSocketRepositoryImpl
        let repository_guard = repository.lock().unwrap();

        // Call the get_listener method on the inner ServerSocketRepositoryImpl
        let listener = repository_guard.get_listener();

        // Assert that the listener is None
        assert!(listener.is_none());

        // Additional test logic if needed...
    }

    #[tokio::test]
    async fn test_get_listener_bound() {
        let repository = Arc::new(Mutex::new(ServerSocketRepositoryImpl::new()));
        let address = "127.0.0.1:9787";

        // Lock the mutex to get a mutable reference to ServerSocketRepositoryImpl
        let mut repository_guard = repository.lock().unwrap();

        // Call the bind_socket method on the inner ServerSocketRepositoryImpl
        match repository_guard.bind_socket(address).await {
            Ok(()) => {
                let listener = repository_guard.get_listener();
                assert!(listener.is_some());
            }
            Err(err) => {
                // The bind_socket method returned an error
                panic!("Error binding socket: {:?}", err);
            }
        }

    }
}

