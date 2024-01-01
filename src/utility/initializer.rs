use crate::client_socket_accept::controller::client_socket_accept_controller_impl::get_client_socket_accept_controller;
use crate::server_socket::service::ServerSocketServiceImpl::get_server_socket_service;
use crate::thread_control::service::thread_worker_service_impl::get_thread_worker_service;

pub struct DomainInitializer;

impl DomainInitializer {
    pub fn init_server_socket_domain(&self) {
        let _ = get_server_socket_service();
    }

    pub fn init_thread_control_domain(&self) {
        let _ = get_thread_worker_service();
    }

    pub fn init_client_socket_accept_domain(&self) {
        let _ = get_client_socket_accept_controller();
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use super::*;

    #[test]
    fn test_server_socket_service_initialization() {
        let domain_initializer = DomainInitializer;
        domain_initializer.init_server_socket_domain();

        let initialized_service = get_server_socket_service();
        assert!(initialized_service.lock().unwrap().is_some());
    }

    #[test]
    fn test_thread_worker_service_initialization() {
        let domain_initializer = DomainInitializer;
        domain_initializer.init_thread_control_domain();

        let initialized_service = get_thread_worker_service();
        assert!(initialized_service.lock().unwrap().is_some());
    }

    #[tokio::test]
    async fn test_get_client_socket_accept_controller() {
        // Initialize the controller using the function
        let controller1 = get_client_socket_accept_controller();

        assert!(controller1.is_some());
    }
}
