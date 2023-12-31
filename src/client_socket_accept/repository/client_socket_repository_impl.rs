use std::collections::HashMap;
use tokio::net::TcpListener;
use std::sync::{Arc, Mutex, Once};
use std::time::Duration;
use async_trait::async_trait;
use lazy_static::lazy_static;
use crate::client_socket_accept::repository::client_socket_repository::ClientSocketRepository;
use crate::server_socket::entity::socket_client::SocketClient;
use crate::server_socket::repository::ServerSocketRepository::ServerSocketRepository;
use crate::server_socket::repository::ServerSocketRepositoryImpl::ServerSocketRepositoryImpl;

pub struct ClientSocketRepositoryImpl {
    client_list: HashMap<String, SocketClient>
}

impl ClientSocketRepositoryImpl {
    pub fn new() -> Self {
        ClientSocketRepositoryImpl {
            client_list: HashMap::new(),
        }
    }
}

#[async_trait]
impl ClientSocketRepository for ClientSocketRepositoryImpl {

    async fn accept_client(&self) {
        // loop {
        //     if let Some(listener) = &self.listener {
        //         match listener.accept().await {
        //             Ok((stream, _)) => {
        //                 // Create a new SocketClient for the accepted client
        //                 let client = SocketClient::new(stream);
        //
        //                 // Add the client to the client list
        //                 self.client_list.lock().unwrap().insert(client.id.clone(), client.clone());
        //             }
        //             Err(err) => {
        //                 // Handle the error (e.g., log or propagate)
        //                 eprintln!("Error accepting client: {:?}", err);
        //             }
        //         }
        //     } else {
        //         // Handle the case when the listener is not yet bound
        //         tokio::time::sleep(Duration::from_secs(1)).await;
        //     }
        // }
    }
}