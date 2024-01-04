use tokio::net::TcpStream;

pub struct SocketClient {
    pub address: String,
    pub stream: TcpStream,
}