use anyhow::Result;
use futures::{SinkExt, StreamExt};
use tokio::net::TcpListener;
use tokio_util::codec::{Framed, LinesCodec};

#[tokio::main]
async fn main() -> Result<()> {
    let addr = "0.0.0.0:8080";
    let listener = TcpListener::bind(addr).await?;
    println!("listen to: {}", addr);

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("accepted connection from {}", addr);
        tokio::spawn(async move {
            // 使用 LinesCodec 把 TCP 数据切成一行行字符串处理
            let framed = Framed::new(socket, LinesCodec::new());
            let (mut w, mut r) = framed.split();
            while let Some(Ok(t)) = r.next().await {
                w.send(format!("I got {}", t)).await?;
            }

            Ok::<_, anyhow::Error>(())
        });
    }
}
