use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;
use tokio::sync::Mutex as AsyncMutex;
use crate::client_socket_accept::repository::client_socket_accept_repository_impl::ClientSocketAcceptRepositoryImpl;
use crate::receiver::repository::server_receiver_repository_impl::ServerReceiverRepositoryImpl;
use crate::receiver::service::server_receiver_service::ServerReceiverService;

pub struct ServerReceiverServiceImpl {
    server_receiver_repository: Arc<AsyncMutex<ServerReceiverRepositoryImpl>>,
    client_socket_accept_repository: Arc<AsyncMutex<ClientSocketAcceptRepositoryImpl>>
}

impl ServerReceiverServiceImpl {
    pub fn new(server_receiver_repository: Arc<AsyncMutex<ServerReceiverRepositoryImpl>>,
               client_socket_accept_repository: Arc<AsyncMutex<ClientSocketAcceptRepositoryImpl>>) -> Self {

        ServerReceiverServiceImpl {
            server_receiver_repository,
            client_socket_accept_repository
        }
    }

    pub fn get_instance() -> Arc<AsyncMutex<ServerReceiverServiceImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<ServerReceiverServiceImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        ServerReceiverServiceImpl::new(
                            ServerReceiverRepositoryImpl::get_instance(),
                            ClientSocketAcceptRepositoryImpl::get_instance())));
        }
        INSTANCE.clone()
    }
}

#[async_trait]
impl ServerReceiverService for ServerReceiverServiceImpl {
    async fn client_receive(&mut self) {
        println!("Server Receiver Service: receive()");

        let client_socket_accept_repository_mutex = ClientSocketAcceptRepositoryImpl::get_instance();
        let client_socket_accept_repository_guard = client_socket_accept_repository_mutex.lock().await;
        let client_list_mutex = client_socket_accept_repository_guard.get_client_list();
        let client_list_guard = client_list_mutex.lock().await;

    }
}
