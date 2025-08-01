use std::collections::VecDeque;
use std::sync::{Condvar, Mutex};

pub struct Channel<T> {
    queue: Mutex<VecDeque<T>>,
    item_ready: Condvar,
}

impl<T> Channel<T> {
    pub fn new() -> Self {
        Channel {
            queue: Mutex::new(VecDeque::new()),
            item_ready: Condvar::new(),
        }
    }

    pub fn send(&self, value: T) {
        let mut queue = self.queue.lock();
        queue.unwrap().push_back(value);
        self.item_ready.notify_one()
    }

    pub fn recv(&self) -> T {
        let mut queue = self.queue.lock().unwrap();

        loop {
            if let Some(msg) = queue.pop_front() {
                return msg;
            }

            queue = self.item_ready.wait(queue).unwrap();
        }
    }
}
