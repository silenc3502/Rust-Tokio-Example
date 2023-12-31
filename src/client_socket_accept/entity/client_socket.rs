use tokio::net::TcpStream;

pub struct ClientSocket {
    address: String,  // 클라이언트의 주소 또는 식별자
    stream: TcpStream,  // 클라이언트와의 연결을 나타내는 TcpStream
}

impl ClientSocket {
    // 생성자
    pub fn new(address: String, stream: TcpStream) -> Self {
        ClientSocket { address, stream }
    }

    // 주소 getter
    pub fn address(&self) -> &str {
        &self.address
    }

    // TcpStream getter
    pub fn stream(&self) -> &TcpStream {
        &self.stream
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::net::{TcpListener, TcpStream};

    #[tokio::test]
    async fn test_socket_client() {
        // Set up a test server to accept connections
        let listener = TcpListener::bind("127.0.0.1:7890").await.unwrap();
        let server_addr = listener.local_addr().unwrap();
        // Connect a client to the test server
        let client_stream = TcpStream::connect(&server_addr).await.unwrap();

        // Create a SocketClient instance
        let client = ClientSocket::new("test_client".to_string(), client_stream);

        // Your test assertions go here
        assert_eq!(client.address(), "test_client");

        // Ensure to gracefully shut down the listener to release the bound port
        drop(listener);
    }
}
