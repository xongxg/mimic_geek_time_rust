use futures::{Stream, StreamExt, stream, stream::poll_fn};
use std::task::Poll;
// use futures_util::{AsyncReadExt, StreamExt, stream};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let mut stream = fib().take(10);

    // let mut stream = fib1(10);

    let mut stream = fib2(10).boxed();

    while let Some(val) = stream.next().await {
        println!("{:?}", val);
    }

    Ok(())
}

fn fib() -> impl Stream<Item = i32> {
    let mut a = 0;
    let mut b = 1;
    let mut t = a;

    stream::repeat_with(move || {
        t = a;
        let c = a + b;
        a = b;
        b = c;
        t
    })
}

fn fib1(mut n: usize) -> impl Stream<Item = i32> {
    let mut a = 0;
    let mut b = 1;
    let mut t = a;

    poll_fn(move |_cx| -> Poll<Option<i32>> {
        if n == 0 {
            return Poll::Ready(None);
        }

        n -= 1;
        t = a;
        let c = a + b;
        a = b;
        b = c;

        Poll::Ready(Some(t))
    })
}

fn fib2(mut n: usize) -> impl Stream<Item = i32> {
    stream::unfold((n-1, (0, 1)), |(mut n, (a, b))| async move {
        if n == 0 {
            None
        } else {
            n -= 1;
            let c = a + b;
            Some((b, (n, (b, c))))
        }
    })
}
