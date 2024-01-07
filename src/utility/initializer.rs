use std::sync::Arc;

use tokio::sync::{mpsc, Mutex};
use tokio::net::TcpStream;

use crate::server_socket::service::server_socket_service_impl::ServerSocketServiceImpl;

use crate::thread_control::service::thread_worker_service_impl::ThreadWorkerServiceImpl;

use crate::client_socket_accept::controller::client_socket_accept_controller_impl::ClientSocketAcceptControllerImpl;
use crate::client_socket_accept::controller::client_socket_accept_controller::ClientSocketAcceptController;

use crate::receiver::controller::server_receiver_controller_impl::ServerReceiverControllerImpl;
use crate::receiver::controller::server_receiver_controller::ServerReceiverController;

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

    pub async fn init_client_socket_accept_domain(&self, acceptor_channel_arc: Arc<AcceptorChannel>) {
        let client_socket_accept_controller_mutex = ClientSocketAcceptControllerImpl::get_instance();
        let client_socket_accept_controller = client_socket_accept_controller_mutex.lock().await;

        client_socket_accept_controller.inject_accept_channel(acceptor_channel_arc).await;
    }

    pub async fn init_receiver_domain(&self, acceptor_channel_arc: Arc<AcceptorChannel>) {
        let server_receiver_controller_mutex = ServerReceiverControllerImpl::get_instance();
        let mut server_receiver_controller = server_receiver_controller_mutex.lock().await;

        server_receiver_controller.inject_accept_channel(acceptor_channel_arc).await;
    }

    pub async fn init_every_domain(&self) {
        let acceptor_channel = AcceptorChannel::new(1);
        let acceptor_channel_arc = Arc::new(acceptor_channel.clone());

        self.init_server_socket_domain();
        self.init_thread_control_domain();
        self.init_client_socket_accept_domain(acceptor_channel_arc.clone()).await;
        self.init_receiver_domain(acceptor_channel_arc.clone()).await;
    }
}

