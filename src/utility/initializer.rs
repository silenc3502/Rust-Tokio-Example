use crate::client_socket_accept::controller::client_socket_accept_controller_impl::ClientSocketAcceptControllerImpl;
use crate::receiver::repository::server_receiver_repository_impl::ServerReceiverRepositoryImpl;
use crate::server_socket::service::server_socket_service_impl::ServerSocketServiceImpl;
use crate::thread_control::service::thread_worker_service_impl::ThreadWorkerServiceImpl;

pub struct DomainInitializer;

impl DomainInitializer {
    pub fn init_server_socket_domain(&self) {
        let _ = ServerSocketServiceImpl::get_instance();
    }

    pub fn init_thread_control_domain(&self) {
        let _ = ThreadWorkerServiceImpl::get_instance();
    }

    pub fn init_client_socket_accept_domain(&self) {
        let _ = ClientSocketAcceptControllerImpl::get_instance();
    }

    pub fn init_receiver_domain(&self) {
        let _ = ServerReceiverRepositoryImpl::get_instance();
    }
}
