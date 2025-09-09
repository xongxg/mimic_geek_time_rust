fn main() {
    let bottle_filler = BottleFillingMachine::new(0);

    // (Mock) Check on some shared and state-specific values
    assert_eq!(
        bottle_filler.state.waiting_time,
        std::time::Duration::new(0, 0)
    );
    assert_eq!(bottle_filler.shared_value, 0);
    // Transition
    let bottle_filler1 = BottleFillingMachine::<Filling>::from(bottle_filler);
    let bottle_filler2 = BottleFillingMachine::<Done>::from(bottle_filler1);

    let bottle_filler3 = BottleFillingMachine::<Done>::from(bottle_filler2);
}

// This is our state machine.
struct BottleFillingMachine<S> {
    shared_value: usize,
    state: S,
}

// The following states can be the 'S' in StateMachine<S>
struct Waiting {
    waiting_time: std::time::Duration,
}

struct Filling {
    rate: usize,
}

struct Done;

impl BottleFillingMachine<Waiting> {
    fn new(shared_value: usize) -> BottleFillingMachine<Waiting> {
        Self {
            shared_value,
            state: Waiting {
                waiting_time: std::time::Duration::new(0, 0),
            },
        }
    }
}

impl From<BottleFillingMachine<Waiting>> for BottleFillingMachine<Filling> {
    fn from(value: BottleFillingMachine<Waiting>) -> Self {
        Self {
            shared_value: value.shared_value,
            state: Filling { rate: 1 },
        }
    }
}

impl From<BottleFillingMachine<Filling>> for BottleFillingMachine<Done> {
    fn from(value: BottleFillingMachine<Filling>) -> Self {
        Self {
            shared_value: value.shared_value,
            state: Done,
        }
    }
}

impl From<BottleFillingMachine<Done>> for BottleFillingMachine<Waiting> {
    fn from(value: BottleFillingMachine<Done>) -> Self {
        Self {
            shared_value: value.shared_value,
            state: Waiting {
                waiting_time: std::time::Duration::new(0, 0),
            },
        }
    }
}

fn transition_the_states(val: BottleFillingMachine<Waiting>) -> BottleFillingMachine<Filling> {
    val.into() // Nice right?
}

