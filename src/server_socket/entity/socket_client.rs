use tokio::net::TcpStream;

pub struct SocketClient {
    pub address: String,  // 클라이언트의 주소 또는 식별자
    pub stream: TcpStream,  // 클라이언트와의 연결을 나타내는 TcpStream
}