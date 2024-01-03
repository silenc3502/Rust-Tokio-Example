use async_trait::async_trait;
use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;
use crate::server_socket::repository::ServerSocketRepository::ServerSocketRepository;
use crate::server_socket::repository::ServerSocketRepositoryImpl::ServerSocketRepositoryImpl;
use crate::server_socket::service::ServerSocketService::ServerSocketService;
use crate::thread_control::repository::thread_worker_repository_impl::ThreadWorkerRepositoryImpl;

#[derive(Clone)]
pub struct ServerSocketServiceImpl {
    repository: Arc<Mutex<ServerSocketRepositoryImpl>>,
}

impl ServerSocketServiceImpl {
    pub fn new(repository: Arc<Mutex<ServerSocketRepositoryImpl>>) -> Self {
        ServerSocketServiceImpl { repository }
    }

    pub fn get_instance() -> Arc<Mutex<ServerSocketServiceImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<Mutex<ServerSocketServiceImpl>> =
                Arc::new(
                    Mutex::new(
                        ServerSocketServiceImpl::new(
                            ServerSocketRepositoryImpl::get_instance())));
        }
        INSTANCE.clone()
    }
}

#[async_trait]
impl ServerSocketService for ServerSocketServiceImpl {
    async fn server_socket_bind(&mut self, address: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut repository_guard = self.repository.lock().unwrap();
        repository_guard.bind_socket(address).await
    }
}

impl AsRef<ServerSocketRepositoryImpl> for ServerSocketRepositoryImpl {
    fn as_ref(&self) -> &ServerSocketRepositoryImpl {
        self
    }
}


#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use super::*;

    #[tokio::test]
    async fn test_get_instance() {
        let mutex_option_repository_direct = get_server_socket_repository();
        let service = get_server_socket_service();

        // Lock the mutex to get a mutable reference to ServerSocketServiceImpl
        let mut service_guard = service.lock().unwrap();

        // Check if the service is initialized
        assert!(service_guard.is_some(), "ServerSocketServiceImpl not initialized");

        if let Some(service_instance) = service_guard.as_mut() {
            let server_socket_repository = &service_instance.repository;
            let option_server_socket_repository = mutex_option_repository_direct.lock().unwrap().clone();
            let converted_server_socket_repository = option_server_socket_repository.unwrap();

            // Check if the repositories match
            assert_eq!(converted_server_socket_repository, server_socket_repository.clone());
        } else {
            // Handle the case when the service is not initialized
            println!("ServerSocketServiceImpl not initialized");
        }
    }

    #[tokio::test]
    async fn test_bind_service() {
        let service = Arc::new(Mutex::new(ServerSocketServiceImpl::new(ServerSocketRepositoryImpl::new())));
        let address = "127.0.0.1:27373";

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

    #[tokio::test]
    async fn test_repository_bind() {
        get_server_socket_service();

        let server_socket_service = ServerSocketServiceImpl::get_instance();
        let mut service_guard = server_socket_service.lock().unwrap();

        assert!(service_guard.is_some(), "ServerSocketServiceImpl not initialized");

        let expected_address = "127.0.0.1:12817";
        match service_guard.as_mut().unwrap().bind(expected_address).await {
            Ok(()) => {
                assert!(service_guard.as_ref().unwrap().repository.get_listener().is_some());

                let tcp_listener = service_guard.as_ref().unwrap().repository.get_listener().unwrap();
                let local_addr = tcp_listener.local_addr().expect("Failed to get local address");

                println!("Bound to address: {}", local_addr);

                assert_eq!(local_addr.to_string(), expected_address);
            }
            Err(err) => {
                // The bind method returned an error
                panic!("Error binding socket: {:?}", err);
            }
        }
    }
}
