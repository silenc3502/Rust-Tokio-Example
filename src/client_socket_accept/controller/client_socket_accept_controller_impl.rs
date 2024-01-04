use std::sync::{Arc, Mutex, Once};
use async_trait::async_trait;
use lazy_static::lazy_static;
use tokio::sync::Mutex as AsyncMutex;
use crate::client_socket_accept::controller::client_socket_accept_controller::ClientSocketAcceptController;
use crate::client_socket_accept::service::client_socket_accept_service::ClientSocketAcceptService;
use crate::client_socket_accept::service::client_socket_accept_service_impl::ClientSocketAcceptServiceImpl;
use crate::server_socket::repository::ServerSocketRepositoryImpl::ServerSocketRepositoryImpl;

#[derive(Clone)]
pub struct ClientSocketAcceptControllerImpl {
    service: Arc<AsyncMutex<ClientSocketAcceptServiceImpl>>,
}

impl ClientSocketAcceptControllerImpl {
    pub fn new(service: Arc<AsyncMutex<ClientSocketAcceptServiceImpl>>) -> Self {
        ClientSocketAcceptControllerImpl { service }
    }

    pub fn get_instance() -> Arc<AsyncMutex<ClientSocketAcceptControllerImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<ClientSocketAcceptControllerImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        ClientSocketAcceptControllerImpl::new(
                            ClientSocketAcceptServiceImpl::get_instance())));
        }
        INSTANCE.clone()
    }
}

#[async_trait]
impl ClientSocketAcceptController for ClientSocketAcceptControllerImpl {
    async fn accept_client(&self) {
        println!("Client Socket Accept Controller: accept()");
        let client_socket_accept_guard = self.service.lock().await;
        client_socket_accept_guard.accept_client().await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::Duration;
    use crate::server_socket::repository::ServerSocketRepository::ServerSocketRepository;
    use crate::server_socket::repository::ServerSocketRepositoryImpl::ServerSocketRepositoryImpl;


    #[tokio::test]
    async fn test_controller_singleton_behavior() {
        let controller1 = ClientSocketAcceptControllerImpl::get_instance();
        let controller2 = ClientSocketAcceptControllerImpl::get_instance();

        assert!(Arc::ptr_eq(&controller1, &controller2));
    }

    #[tokio::test]
    async fn test_controller_accept_client() {
        let client_socket_accept_controller_mutex = ClientSocketAcceptControllerImpl::get_instance();
        let server_socket_service_mutex = ServerSocketRepositoryImpl::get_instance();

        tokio::spawn(async move {
            let client_socket_accept_controlelr_gaurd = client_socket_accept_controller_mutex.lock().await;

            let mut server_socket_service_guard = server_socket_service_mutex.lock().await;
            let address = "127.0.0.1:7373";
            server_socket_service_guard.bind_socket(address).await.expect("TODO: bind failed");
            
            let listener_option = server_socket_service_guard.get_listener().await;
            assert!(listener_option.is_some());

            drop(server_socket_service_guard);

            // Call accept_client on the controller
            client_socket_accept_controlelr_gaurd.accept_client().await;
        });

        // Sleep for a short duration to allow the async method to run
        tokio::time::sleep(Duration::from_secs(5)).await;

        // The test passes if it reaches this point without panicking
    }

    #[tokio::test]
    async fn test_get_client_socket_accept_controller() {
        let controller1_mutex = ClientSocketAcceptControllerImpl::get_instance();
        let controller2_mutex = ClientSocketAcceptControllerImpl::get_instance();

        let controller1_guard = controller1_mutex.lock().await;
        let controller2_guard = controller2_mutex.lock().await;

        assert!(Arc::ptr_eq(&controller1_mutex, &controller2_mutex));
    }
}