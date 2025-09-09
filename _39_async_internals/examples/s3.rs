use async_std::task;
use std::cell::Cell;
use std::pin::Pin;
use std::ptr::null;
use std::rc::Rc;
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
use std::time::Duration;
use tokio::time::error::Elapsed;

fn main() {
    let elapsed = Rc::new(Cell::new(0));
    let task13 = Ticket::new(13, elapsed.clone());
    let task19 = Ticket::new(19, elapsed.clone());

    let waker = RawWaker::new(null(), &VTABLE);
    let waker = unsafe { Waker::from_raw(waker) };

    let mut cx = Context::from_waker(&waker);
    let mut task13 = Box::pin(task13);
    let mut task19 = Box::pin(task19);

    loop {
        match task13.as_mut().poll(&mut cx) {
            Poll::Ready(result) => break println!("{:>4} ready {result:?}", elapsed.get()),
            Poll::Pending => println!("{:>4} Pending 13", elapsed.get()),
        }

        match task19.as_mut().poll(&mut cx) {
            Poll::Ready(value) => break println!("{:>4} ready {value:?}", elapsed.get()),
            Poll::Pending => println!("{:>4} pending 19", elapsed.get()),
        }

        std::thread::sleep(Duration::from_millis(300));
        elapsed.set(elapsed.get() + 300);
    }

    println!("{:>4} completed", elapsed.get())
}

fn wake(_data: *const ()) {}
fn noop(_data: *const ()) {}

static VTABLE: RawWakerVTable =
    RawWakerVTable::new(|data| RawWaker::new(data, &VTABLE), wake, wake, noop);

pub struct Resource {
    value: i32,
    elapsed: Rc<Cell<i32>>,
}

impl Resource {
    pub fn result(&self) -> i32 {
        self.value
    }
}

impl Drop for Resource {
    fn drop(&mut self) {
        println!("{:>4} Dropped {}", self.elapsed.get(), self.value);
    }
}

pub struct Ticket {
    timer: Pin<Box<dyn Future<Output = ()>>>,
    resource: Resource,
}

impl Ticket {
    pub fn new(value: i32, elapsed: Rc<Cell<i32>>) -> Self {
        Self {
            timer: Box::pin(task::sleep(Duration::from_secs(1))),
            resource: Resource { value, elapsed },
        }
    }
}

impl Future for Ticket {
    type Output = i32;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.timer.as_mut().poll(cx) {
            Poll::Ready(_) => Poll::Ready(self.resource.value),
            Poll::Pending => Poll::Pending,
        }
    }
}
