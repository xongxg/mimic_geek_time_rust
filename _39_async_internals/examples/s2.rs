use async_std::task;
use std::cell::Cell;
use std::ptr::null;
use std::rc::Rc;
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
use std::time::Duration;

fn main() {
    let elapsed = Rc::new(Cell::new(0));
    let copied13 = elapsed.clone();
    let task13 = async {
        let resource = Resource {
            value: 13,
            elapsed: copied13,
        };
        task::sleep(Duration::from_secs(1)).await;
        resource.result()
    };

    let copied19 = elapsed.clone();
    let task19 = async {
        let resource = Resource {
            value: 19,
            elapsed: copied19,
        };
        task::sleep(Duration::from_secs(2)).await;
        resource.result()
    };

    let waker = RawWaker::new(null(), &VTABLE);
    let waker = unsafe { Waker::from_raw(waker) };

    let mut context = Context::from_waker(&waker);
    let mut task13 = Box::pin(task13);
    let mut task19 = Box::pin(task19);

    loop {
        match task13.as_mut().poll(&mut context) {
            Poll::Ready(result) => break println!("{:>4} ready {result:?}", elapsed.get()),
            Poll::Pending => println!("{:>4} Pending 13", elapsed.get()),
        }

        match task19.as_mut().poll(&mut context) {
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

