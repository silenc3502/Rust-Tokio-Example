use std::sync::Arc;
use crate::client_socket_accept::controller::client_socket_accept_controller_impl::ClientSocketAcceptControllerImpl;
use crate::receiver::repository::server_receiver_repository_impl::ServerReceiverRepositoryImpl;
use crate::server_socket::service::server_socket_service_impl::ServerSocketServiceImpl;
use crate::thread_control::service::thread_worker_service_impl::ThreadWorkerServiceImpl;

use tokio::sync::{mpsc, Mutex};
use futures::future::err;
use tokio::net::TcpStream;

use crate::utility::mpsc::mpsc_creator::mpsc_channel::define_channel;

define_channel!(AcceptorChannel, Arc<Mutex<TcpStream>>);

pub struct DomainInitializer;

impl DomainInitializer {
    pub fn init_server_socket_domain(&self) {
        let _ = ServerSocketServiceImpl::get_instance();
    }

    pub fn init_thread_control_domain(&self) {
        let _ = ThreadWorkerServiceImpl::get_instance();
    }

    pub fn init_client_socket_accept_domain(&self, acceptor_channel_arc: Arc<AcceptorChannel>) {
        let _ = ClientSocketAcceptControllerImpl::get_instance();
    }

    pub fn init_receiver_domain(&self) {
        let _ = ServerReceiverRepositoryImpl::get_instance();
    }

    pub fn init_every_domain(&self) {
        let acceptor_channel = AcceptorChannel::new(1);
        let acceptor_channel_arc = Arc::new(acceptor_channel.clone());

        self.init_server_socket_domain();
        self.init_thread_control_domain();
        self.init_client_socket_accept_domain(acceptor_channel_arc.clone());
    }
}

