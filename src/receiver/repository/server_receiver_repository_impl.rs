use std::str::from_utf8;
use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;
use tokio::sync::Mutex as AsyncMutex;
use crate::receiver::entity::receive_data::ReceiveData;
use crate::receiver::repository::server_receiver_repository::ServerReceiverRepository;

pub struct ServerReceiverRepositoryImpl {
    receive_data: ReceiveData
}

impl ServerReceiverRepositoryImpl {
    pub fn new() -> Self {
        ServerReceiverRepositoryImpl {
            receive_data: ReceiveData::new()
        }
    }

    pub fn get_instance() -> Arc<AsyncMutex<ServerReceiverRepositoryImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<ServerReceiverRepositoryImpl>> =
                Arc::new(AsyncMutex::new(ServerReceiverRepositoryImpl::new()));
        }
        INSTANCE.clone()
    }

    pub fn get_receive_data(&self) -> &ReceiveData {
        &self.receive_data
    }
}

#[async_trait]
impl ServerReceiverRepository for ServerReceiverRepositoryImpl {
    async fn receive(&mut self, mut stream: TcpStream) {
        println!("Server Receiver Repository: receive()");

        if let Ok(peer_addr) = stream.peer_addr() {
            println!("Connected client address: {}", peer_addr);
        }

        while let Ok(bytes_read) = stream.read(self.receive_data.receive_content_mut()).await {
            if bytes_read == 0 {
                break;
            }

            let stored_data = self.receive_data.get_receive_content();
            if let Ok(utf8_string) = from_utf8(&stored_data[..bytes_read]) {
                println!("Received content: {}", utf8_string);
            } else {
                println!("Received content is not a valid UTF-8 string");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;
    use tokio::io::AsyncWriteExt;
    use super::*;
    use tokio::net::{TcpListener, TcpStream};

    #[tokio::test]
    async fn test_server_receiver_repository_receive() {
        let repository = ServerReceiverRepositoryImpl::new();
        let repository_mutex = Arc::new(AsyncMutex::new(repository));

        let receiver_thread = tokio::spawn(async move {
            let listener = TcpListener::bind("192.168.20.2:12345").await.expect("Failed to bind address");
            println!("server: bind Success");

            let (mut stream, _) = listener.accept().await.expect("Failed to accept connection");
            println!("server: accept Success");

            let mut repository_guard = repository_mutex.lock().await;
            repository_guard.receive(stream).await;
        });

        tokio::time::sleep(Duration::from_secs(2)).await;

        let transmitter_thread = tokio::spawn(async move {
            match TcpStream::connect("192.168.20.2:12345").await {
                Ok(mut stream) => {
                    println!("Success to connect!");

                    let data_to_send = b"Hello, Rust Network Library: Tokio!";
                    if let Err(e) = stream.write_all(&data_to_send[..]).await {
                        eprintln!("Failed to write to stream: {:?}", e);
                    }
                }
                _ => {}
            }
        });

        tokio::try_join!(receiver_thread, transmitter_thread).expect("Test failed");
    }
}
