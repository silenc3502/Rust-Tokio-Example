use tokio::sync::{mpsc, Mutex};

macro_rules! define_channel {
    ($struct_name:ident, $type:ty) => {
        struct $struct_name {
            sender: mpsc::Sender<$type>,
            receiver: Mutex<mpsc::Receiver<$type>>,
        }

        impl $struct_name {
            fn new(capacity: usize) -> Self {
                let (sender, receiver) = tokio::sync::mpsc::channel::<$type>(capacity);
                $struct_name { sender, receiver: Mutex::new(receiver) }
            }

            async fn send(&self, value: $type) {
                if let Err(_) = self.sender.send(value).await {

                }
            }

            async fn receive(&self) -> Option<$type> {
                self.receiver.lock().await.recv().await
            }
        }
    };
}



#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::Duration;
    use std::sync::Arc;
    use tokio::io::AsyncReadExt;
    use tokio::net::{TcpListener, TcpStream};

    #[tokio::test]
    async fn test_socket_communication() {
        define_channel!(Acceptor, TcpStream);
        define_channel!(Receiver, String);
        define_channel!(Transmitter, String);

        async fn server(acceptor: Acceptor, transmitter: Transmitter) {
            let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
            println!("Server listening on 127.0.0.1:8080");

            while let Ok((stream, _)) = listener.accept().await {
                let acceptor_clone = acceptor.clone();
                let transmitter_clone = transmitter.clone();

                tokio::spawn(async move {
                    acceptor_clone.send(stream).await;

                    // Simulate sending data from the server
                    for i in 0..5 {
                        let message = format!("Server sending: {}", i);
                        println!("{}", message);
                        transmitter_clone.send(message).await;
                        tokio::time::sleep(Duration::from_secs(1)).await;
                    }
                });
            }
        }

        async fn client(receiver: Receiver) {
            while let Some(stream) = receiver.receive().await {
                tokio::spawn(async move {
                    let mut buf = [0; 1024];
                    let mut stream = tokio::net::TcpStream::from_std(stream).unwrap();

                    loop {
                        let n = stream.read(&mut buf).await.unwrap();
                        if n == 0 {
                            break;
                        }

                        let message = String::from_utf8_lossy(&buf[..n]).into_owned();
                        println!("Client received: {}", message);
                    }
                });
            }
        }

        // Create channels for communication
        let (acceptor_tx, acceptor_rx) = mpsc::channel::<TcpStream>(10);
        let acceptor = Acceptor::new(acceptor_tx);
        let receiver = Receiver::new(acceptor_rx);
        let transmitter = Transmitter::new();

        // Spawn server
        let server_task = tokio::spawn(server(acceptor, transmitter));

        // Spawn client
        let client_task = tokio::spawn(client(receiver));

        // Give some time for tasks to complete
        tokio::time::sleep(Duration::from_secs(5)).await;

        // Check if server and client tasks completed successfully
        assert!(server_task.await.is_ok());
        assert!(client_task.await.is_ok());
    }

    #[tokio::test]
    async fn test_channel_send_receive() {
        define_channel!(TestChannel, usize);
        let channel = Arc::new(TestChannel::new(10));

        let channel_clone = Arc::clone(&channel);
        tokio::spawn(async move {
            channel_clone.send(42).await;
        });

        let channel_clone = Arc::clone(&channel);
        tokio::spawn(async move {
            if let Some(value) = channel_clone.receive().await {
                println!("Received value: {}", value);
                assert_eq!(value, 42);
            } else {
                println!("No value received");
            }
        });

        // Wait for tasks to complete
        tokio::time::sleep(Duration::from_secs(2)).await;
    }
}
