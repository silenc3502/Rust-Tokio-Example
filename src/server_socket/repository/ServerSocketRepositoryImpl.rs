use tokio::net::TcpListener;
use std::sync::{Arc, Mutex, Once};
use async_trait::async_trait;
use lazy_static::lazy_static;
use crate::server_socket::repository::ServerSocketRepository::ServerSocketRepository;

#[derive(Debug)]
pub struct ServerSocketRepositoryImpl {
    listener: Option<TcpListener>,
}

impl Clone for ServerSocketRepositoryImpl {
    fn clone(&self) -> Self {
        ServerSocketRepositoryImpl {
            listener: None,
        }
    }
}

impl Eq for ServerSocketRepositoryImpl {}

impl PartialEq for ServerSocketRepositoryImpl {
    fn eq(&self, other: &Self) -> bool {
        self.listener.is_none() && other.listener.is_none()
    }
}

impl ServerSocketRepositoryImpl {
    pub fn new() -> Self {
        ServerSocketRepositoryImpl {
            listener: None,
        }
    }

    pub fn get_instance() -> &'static Arc<Mutex<ServerSocketRepositoryImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<Mutex<ServerSocketRepositoryImpl>> =
                Arc::new(Mutex::new(ServerSocketRepositoryImpl::new()));
        }
        &INSTANCE
    }
}

#[async_trait]
impl ServerSocketRepository for ServerSocketRepositoryImpl {
    async fn bind_socket(&mut self, address: &str) -> Result<(), Box<dyn std::error::Error>> {
        let listener = TcpListener::bind(address)
            .await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

        self.listener = Some(listener);
        if let Some(listener) = &self.listener {
            println!("Listener bound successfully: {}", listener.local_addr().unwrap());
        }
        Ok(())
    }

    fn get_listener(&self) -> Option<&tokio::net::TcpListener> {
        self.listener.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use super::*;

    #[tokio::test]
    async fn test_bind_socket() {
        let repository = Arc::new(Mutex::new(ServerSocketRepositoryImpl::new()));
        let address = "127.0.0.1:37373";

        let mut repository_guard = repository.lock().unwrap();

        match repository_guard.bind_socket(address).await {
            Ok(()) => {
                if let Some(listener) = repository_guard.get_listener() {
                    let bound_address = listener.local_addr().unwrap();
                    assert_eq!(bound_address.to_string(), address);
                } else {
                    // Listener is not present after successful bind
                    panic!("Listener not found after successful bind");
                }
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
        let repository_guard = repository.lock().unwrap();
        let listener = repository_guard.get_listener();

        assert!(listener.is_none());
    }

    #[tokio::test]
    async fn test_get_listener_bound() {
        let repository = Arc::new(Mutex::new(ServerSocketRepositoryImpl::new()));
        let address = "127.0.0.1:9787";

        let mut repository_guard = repository.lock().unwrap();

        match repository_guard.bind_socket(address).await {
            Ok(()) => {
                let listener = repository_guard.get_listener();
                assert!(listener.is_some());
            }
            Err(err) => {
                panic!("Error binding socket: {:?}", err);
            }
        }
    }

    #[tokio::test]
    async fn test_get_instance() {
        let instance1 = ServerSocketRepositoryImpl::get_instance();
        let instance2 = ServerSocketRepositoryImpl::get_instance();

        assert_eq!(instance1 as *const _, instance2 as *const _);
    }
}

