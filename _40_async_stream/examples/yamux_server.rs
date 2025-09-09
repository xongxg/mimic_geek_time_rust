use anyhow::Result;
use futures::prelude::*;
use tokio::net::TcpListener;
use tokio_util::{
    codec::{Framed, LinesCodec},
    compat::{FuturesAsyncReadCompatExt, TokioAsyncReadCompatExt},
};
use tracing::info;
use yamux::{Config, Connection, Mode, WindowUpdateMode};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    let addr = "0.0.0.0:8080";
    let listener = TcpListener::bind(addr).await?;
    info!("Listening on: {:?}", addr);

    loop {
        let (stream, addr) = listener.accept().await?;
        info!("Got a new connection from {:?}", addr);

        let mut config = Config::default();
        config.set_window_update_mode(WindowUpdateMode::OnRead);
        // 使用 compat() 方法把 tokio AsyncRead/AsyncWrite 转换成 futures 对应的 trait
        let conn = Connection::new(stream.compat(), config, Mode::Server);
        // Yamux ctrl stream 可以用来打开新的 stream
        let _ctrl = conn.control();

        tokio::spawn(async move {
            yamux::into_stream(conn).try_for_each_concurrent(None, move |s| async {
                let mut framed = Framed::new(s.compat(), LinesCodec::new());
                while let Some(Ok(line)) = framed.next().await {
                    println!("Got: {}", line);
                    framed
                        .send(format!("Hello! I got '{}'", line))
                        .await
                        .unwrap();
                }

                Ok(())
            }).await.expect("TODO: panic message");
        });
    }
}
