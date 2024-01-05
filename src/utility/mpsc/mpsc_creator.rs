use tokio::sync::{mpsc, Mutex};
use futures::future::err;

pub mod mpsc_channel {
    #[macro_export]
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
                    if let Err(err) = self.sender.send(value).await {
                        eprintln!("Error sending message: {}", err);
                    }
                }

                async fn receive(&self) -> Option<$type> {
                    self.receiver.lock().await.recv().await
                }
            }

            impl Clone for $struct_name {
                fn clone(&self) -> Self {
                    let (sender, receiver) = tokio::sync::mpsc::channel::<$type>(self.sender.capacity());
                    $struct_name {
                        sender,
                        receiver: Mutex::new(receiver),
                    }
                }
            }
        };
    }
    pub use crate::define_channel;
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::{Duration, sleep};
    use std::sync::Arc;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::{TcpListener, TcpStream};
    pub use crate::utility::mpsc::mpsc_creator::mpsc_channel::define_channel;

    #[tokio::test]
    async fn test_socket_communication() {
        define_channel!(Acceptor, Arc<Mutex<TcpStream>>);

        async fn acceptor_thread(acceptor: Arc<Acceptor>) {
            let listener = TcpListener::bind("192.168.20.2:12123").await.unwrap();
            println!("Server listening on 192.168.20.2:12123");

            while let Ok((stream, _)) = listener.accept().await {
                println!("Client Accepted!");
                let acceptor_clone = acceptor.clone();

                tokio::spawn(async move {
                    let stream = Arc::new(Mutex::new(stream));
                    acceptor_clone.send(stream).await;
                    println!("TcpStream sent to DataProcessor");
                });
            }
        }

        async fn data_processor_thread(acceptor: Arc<Acceptor>) {
            println!("Waiting for Acceptor receive TcpStream");
            while let Some(arc_mutex_stream) = acceptor.receive().await {
                println!("Waiting for Client Accept!");
                tokio::spawn(async move {
                    let mut stream = arc_mutex_stream.lock().await;

                    loop {
                        let mut buf = [0; 1024];
                        match stream.read(&mut buf).await {
                            Ok(n) if n > 0 => {
                                let message = String::from_utf8_lossy(&buf[..n]).into_owned();
                                println!("Received: {}", message);

                                stream.write_all(&buf[..n]).await.unwrap();
                            }
                            Ok(_) | Err(_) => {
                                break;
                            }
                        }
                    }
                });
            }
        }

        let acceptor = Acceptor::new(1);
        let acceptor_arc = Arc::new(acceptor.clone());

        tokio::spawn(acceptor_thread(acceptor_arc.clone()));
        tokio::spawn(data_processor_thread(acceptor_arc.clone()));

        sleep(Duration::from_secs(1)).await;

        let mut client_stream = TcpStream::connect("192.168.20.2:12123").await;
        println!("Client Connect Success!");

        if let Ok(ref mut stream) = client_stream {
            println!("Ready to send Message!");
            stream.write_all(b"Hello Rust Tokio MPSC").await.unwrap();
        } else {
            println!("Failed to obtain TcpStream");
        }

        sleep(Duration::from_secs(3)).await;

        println!("Test finished");
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

        tokio::time::sleep(Duration::from_secs(2)).await;
    }
}
