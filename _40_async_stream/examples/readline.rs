use futures::{Stream, StreamExt};
use pin_project::pin_project;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::fs;
use tokio::io::{AsyncBufReadExt, AsyncRead, BufReader, Lines};

#[pin_project]
pub struct LineStream<R> {
    #[pin]
    lines: Lines<BufReader<R>>,
}

impl<R: AsyncRead> LineStream<R> {
    pub fn new(reader: BufReader<R>) -> Self {
        Self {
            lines: reader.lines(),
        }
    }
}

impl<R: AsyncRead> Stream for LineStream<R> {
    type Item = Result<String, std::io::Error>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.project()
            .lines
            .poll_next_line(cx)
            .map(Result::transpose)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = fs::File::open("Cargo.toml").await?;
    let reader = BufReader::new(file);
    let mut st = LineStream::new(reader);
    while let Some(Ok(line)) = st.next().await {
        println!("Got: {}", line);
    }

    Ok(())
}
