fn main() {
    let in_waiting_state = Waiting::new();
    let in_filling_state = in_waiting_state.to_filling();
}

trait SharedFunctionality {
    fn get_shared_value(&self) -> usize;
}

struct Waiting {
    waiting_time: std::time::Duration,
    // Value shared by all states.
    shared_value: usize,
}

impl Waiting {
    fn new() -> Self {
        Waiting {
            waiting_time: std::time::Duration::new(0,0),
            shared_value: 0,
        }
    }
    // Consumes the value!
    fn to_filling(self) -> Filling {
        Filling {
            rate: 1,
            shared_value: 0,
        }
    }
}

impl SharedFunctionality for Waiting {
    fn get_shared_value(&self) -> usize {
        self.shared_value
    }
}

struct Filling {
    rate: usize,
    // Value shared by all states.
    shared_value: usize,
}
impl SharedFunctionality for Filling {
    fn get_shared_value(&self) -> usize {
        self.shared_value
    }
}

