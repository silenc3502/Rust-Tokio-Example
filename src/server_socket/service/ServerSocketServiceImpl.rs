use async_trait::async_trait;
use std::sync::{Arc, Mutex, Once};
use lazy_static::lazy_static;
use crate::server_socket::repository::ServerSocketRepository::ServerSocketRepository;
use crate::server_socket::repository::ServerSocketRepositoryImpl::ServerSocketRepositoryImpl;
use crate::server_socket::service::ServerSocketService::ServerSocketService;

pub struct ServerSocketServiceImpl {
    repository: ServerSocketRepositoryImpl,
}

impl ServerSocketServiceImpl {
    pub fn new(repository: ServerSocketRepositoryImpl) -> Self {
        ServerSocketServiceImpl { repository }
    }
}

#[async_trait]
impl ServerSocketService for ServerSocketServiceImpl {
    async fn bind(&mut self, address: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.repository.bind_socket(address).await
    }
}

lazy_static! {
    static ref SERVER_SOCKET_SERVICE: Mutex<Option<ServerSocketServiceImpl>> = Mutex::new(None);
    static ref INIT: Once = Once::new();
}

pub fn get_server_socket_service() -> &'static Mutex<Option<ServerSocketServiceImpl>> {
    INIT.call_once(|| {
        let repository = ServerSocketRepositoryImpl::new();
        let service = ServerSocketServiceImpl::new(repository);
        *SERVER_SOCKET_SERVICE.lock().unwrap() = Some(service);
    });
    &SERVER_SOCKET_SERVICE
}


#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::Duration;

    #[tokio::test]
    async fn test_bind_service() {
        let service = Arc::new(Mutex::new(ServerSocketServiceImpl::new(ServerSocketRepositoryImpl::new())));
        let address = "127.0.0.1:7373";

        // Lock the mutex to get a mutable reference to ServerSocketServiceImpl
        let mut service_guard = service.lock().unwrap();

        // Call the bind method on the inner ServerSocketServiceImpl
        match service_guard.bind(address).await {
            Ok(()) => {
                // The bind method returned Ok
                // Additional test logic if needed...

                // Assert that the listener is present
                assert!(service_guard.repository.get_listener().is_some());

            }
            Err(err) => {
                // The bind method returned an error
                panic!("Error binding socket: {:?}", err);
            }
        }
    }
}
