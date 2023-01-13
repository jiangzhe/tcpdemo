use smol::net::TcpListener;
use smol::io::{AsyncReadExt, AsyncWriteExt};
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    smol::block_on(async {
        let listener = TcpListener::bind("127.0.0.1:8080").await?;
        loop {
            let (mut socket, _) = listener.accept().await?;

            smol::spawn(async move {
                let start = Instant::now();
                let mut reqs = 0usize;
                let mut buf = [0; 1];

                // In a loop, read data from the socket and write the data back.
                loop {
                    let n = match socket.read(&mut buf).await {
                        // socket closed
                        Ok(0) | Err(_) => {
                            let dur = start.elapsed();
                            let qps = reqs as f64 / dur.as_millis() as f64 * 1000.0;
                            println!("Closed connection. Received {} requests. Duration {:?}. QPS {}", reqs, dur, qps);
                            return;
                        }
                        Ok(n) => n,
                    };

                    reqs += 1;

                    // Write the data back
                    if let Err(e) = socket.write_all(&buf[0..n]).await {
                        eprintln!("failed to write to socket; err = {:?}", e);
                        return;
                    }
                }
            }).detach();
        }
    })
}
