use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

const REQ_COUNT: usize = 200_000;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut socket = TcpStream::connect("127.0.0.1:8080").await?;
    socket.set_nodelay(true).unwrap();
    let mut buf = [0; 1];
    for _ in 0..REQ_COUNT {
        socket.write_all(&buf[..]).await?;
        let n = socket.read(&mut buf).await?;
        assert!(n == 1);
    }
    Ok(())
}