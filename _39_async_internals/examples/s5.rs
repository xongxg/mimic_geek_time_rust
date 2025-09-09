use std::mem;
use std::pin::Pin;
use std::task::{Context, Poll};

fn main() {
    let my_future = NestedEnumFuture::Start;
    let result = futures::executor::block_on(my_future);
    println!("Final result: {}", result);
}

// A dummy future that simulates a real I/O operation.
// We'll use this as our "nested" future.
struct DummyIoFuture {
    value: u32,
    ready: bool,
}

impl Future for DummyIoFuture {
    type Output = u32;

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.ready {
            Poll::Ready(self.value)
        } else {
            // In a real future, you would use the `cx` to wake the task.
            // For this example, we just simulate the state change.
            self.ready = true;
            Poll::Pending
        }
    }
}

// The main, nested state machine future.
enum NestedEnumFuture {
    // Initial state before the first operation.
    Start,
    // The state while the first operation is running.
    Fetching(DummyIoFuture),
    // The state for the second operation, after fetching is complete.
    Processing(u32),
    // The final state, after all operations are complete.
    Finished,
}

impl Future for NestedEnumFuture {
    type Output = u32;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        loop {
            let this = self.as_mut().get_mut();

            match *this {
                NestedEnumFuture::Start => {
                    println!("State: Start -> Fetching");
                    *this = NestedEnumFuture::Fetching(DummyIoFuture {
                        value: 10,
                        ready: false,
                    });
                }
                NestedEnumFuture::Fetching(ref mut future) => {
                    println!("State: Polling inner Fetching future");
                    // Pinning the inner future for the poll call.
                    let inner_fut = unsafe { Pin::new_unchecked(future) };
                    match inner_fut.poll(cx) {
                        Poll::Pending => {
                            println!("State: Fetching -> Pending");
                            return Poll::Pending;
                        }
                        Poll::Ready(value) => {
                            println!("State: Fetching -> Processing with value {}", value);
                            *this = NestedEnumFuture::Processing(value);
                        }
                    }
                }
                NestedEnumFuture::Processing(n) => {
                    println!("State: Processing d ata: {}", n);
                    let result = n * 2;
                    println!("State: Processing -> Finished");
                    *this = NestedEnumFuture::Finished;
                    return Poll::Ready(result);
                }
                NestedEnumFuture::Finished => {
                    println!("State: Finished (already completed)");
                    // A defensive check against re-polling.
                    return Poll::Ready(0);
                }
            }
        }
    }
}
