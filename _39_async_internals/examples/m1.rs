use std::time::Duration;

fn main() {
    let mut state_machine = StateMachine::new();
    state_machine.to_filling();
}

enum State {
    Waiting { waiting_time: Duration },
    Filling { rate: usize },
    Done,
}

struct StateMachine {
    state: State,
}

impl StateMachine {
    pub fn new() -> StateMachine {
        Self {
            state: State::Waiting {
                waiting_time: Duration::new(0, 0),
            },
        }
    }

    pub fn to_filling(&mut self) {
        self.state = match self.state {
            State::Waiting { waiting_time } => State::Filling { rate: 1 },
            _ => panic!(),
        }
    }
}
