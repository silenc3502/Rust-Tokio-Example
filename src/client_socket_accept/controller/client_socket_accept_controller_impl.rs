use std::sync::{Arc, Mutex};
use async_trait::async_trait;
use lazy_static::lazy_static;
use crate::client_socket_accept::controller::client_socket_accept_controller::ClientSocketAcceptController;
use crate::client_socket_accept::service::client_socket_accept_service::ClientSocketAcceptService;
use crate::client_socket_accept::service::client_socket_accept_service_impl::ClientSocketAcceptServiceImpl;

#[derive(Clone)]
pub struct ClientSocketAcceptControllerImpl {
    service: Arc<dyn ClientSocketAcceptService>,
}

impl ClientSocketAcceptControllerImpl {
    pub fn new(service: Arc<dyn ClientSocketAcceptService>) -> Self {
        ClientSocketAcceptControllerImpl { service }
    }

    pub fn get_instance() -> Arc<ClientSocketAcceptControllerImpl> {
        CLIENT_SOCKET_ACCEPT_CONTROLLER_INSTANCE.lock().unwrap().clone()
    }
}

#[async_trait]
impl ClientSocketAcceptController for ClientSocketAcceptControllerImpl {
    async fn accept_client(&self) {
        println!("Client Socket Accept Controller: accept()");
        self.service.accept_client().await;
    }
}

lazy_static! {
    static ref CLIENT_SOCKET_ACCEPT_CONTROLLER_INSTANCE: Mutex<Arc<ClientSocketAcceptControllerImpl>> = Mutex::new(Arc::new(ClientSocketAcceptControllerImpl::new(ClientSocketAcceptServiceImpl::get_instance())));
}

pub fn get_client_socket_accept_controller() -> Option<Arc<ClientSocketAcceptControllerImpl>> {
    let instance = CLIENT_SOCKET_ACCEPT_CONTROLLER_INSTANCE.lock().unwrap();
    Some(instance.clone())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::Duration;
    use tokio::net::TcpListener;
    use crate::client_socket_accept::repository::client_socket_accept_repository_impl::ClientSocketAcceptRepositoryImpl;
    use crate::server_socket::repository::ServerSocketRepository::ServerSocketRepository;
    use crate::server_socket::repository::ServerSocketRepositoryImpl::ServerSocketRepositoryImpl;

    #[tokio::test]
    async fn test_controller_singleton_behavior() {
        // Set up your controller instance
        let controller1 = ClientSocketAcceptControllerImpl::get_instance();
        let controller2 = ClientSocketAcceptControllerImpl::get_instance();

        // Ensure that the instances are the same
        assert!(Arc::ptr_eq(&controller1, &controller2));
    }

    #[tokio::test]
    async fn test_controller_accept_client() {
        let mut server_socket_repository = ServerSocketRepositoryImpl::new();
        let address = "127.0.0.1:7373";
        let result = server_socket_repository.bind_socket(address).await;

        assert!(result.is_ok());
        assert!(server_socket_repository.get_listener().is_some());

        let controller = ClientSocketAcceptControllerImpl::get_instance();
        let service = ClientSocketAcceptServiceImpl::get_instance();
        let repository = ClientSocketAcceptRepositoryImpl::get_instance();

        // Use Tokio's spawn to run the async method
        tokio::spawn(async move {
            // Call accept_client on the controller
            controller.accept_client().await;

            // You may want to add assertions or checks based on the behavior you expect
        });

        // Sleep for a short duration to allow the async method to run
        tokio::time::sleep(Duration::from_secs(30)).await;

        // The test passes if it reaches this point without panicking
    }

    #[tokio::test]
    async fn test_get_client_socket_accept_controller() {
        let controller1 = get_client_socket_accept_controller();
        let controller2 = get_client_socket_accept_controller();
        assert!(Arc::ptr_eq(&controller1.unwrap(), &controller2.unwrap()));
    }
}
