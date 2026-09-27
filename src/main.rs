use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6667").await?;

    loop {
        let (mut socket, addr) = listener.accept().await?;
        println!("accepted connection from {:?}", addr);

        tokio::spawn(async move {
            let mut buf = [0u8; 1024];
            loop {
                match socket.read(&mut buf).await {
                    Ok(0) => break,
                    Ok(n) => {
                        println!("{} bytes read", n);
                        println!("{}", std::str::from_utf8(&buf[..n]).unwrap());
                        socket.write_all(&buf[..n]).await.unwrap();
                    }
                    Err(_) => break,
                }
            }
        });
    }
}
