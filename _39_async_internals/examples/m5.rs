fn main() {
    // The `<StateA>` is implied here. We don't need to add type annotations!
    let in_state_a = StateMachine::new("Blah blah blah".into());

    // This is okay here. But later once we've changed state it won't work anymore.
    in_state_a.some_unrelated_value;
    println!("Starting Value: {}", in_state_a.state.start_value);

    // Transition to the new state. This consumes the old state.
    // Here we need type annotations (since not all StateMachines are linear in their state).
    let in_state_b = StateMachine::<StateB>::from(in_state_a);
    // And our final state.
    let in_state_c = StateMachine::<StateC>::from(in_state_b);

    // This doesn't work either! The state doesn't even contain this value.
    // in_state_c.state.start_value;

    println!("Final state: {}", in_state_c.state.final_value);
}

pub struct StateMachine<S> {
    state: S,
    some_unrelated_value: usize,
}

pub struct StateA {
    start_value: String,
}

impl StateA {
    pub fn new(val: String) -> StateA {
        Self { start_value: val }
    }
}

impl StateMachine<StateA> {
    pub fn new(val: String) -> StateMachine<StateA> {
        Self {
            state: StateA::new(val),
            some_unrelated_value: 0,
        }
    }
}

impl From<StateMachine<StateA>> for StateMachine<StateB> {
    fn from(value: StateMachine<StateA>) -> Self {
        Self {
            state: StateB {
                interm_value: value
                    .state
                    .start_value
                    .split(" ")
                    .map(|x| x.into())
                    .collect(),
            },
            some_unrelated_value: value.some_unrelated_value,
        }
    }
}

pub struct StateB {
    interm_value: Vec<String>,
}

impl From<StateMachine<StateB>> for StateMachine<StateC> {
    fn from(value: StateMachine<StateB>) -> Self {
        Self {
            state: StateC {
                final_value: value.state.interm_value.len(),
            },
            some_unrelated_value: value.some_unrelated_value,
        }
    }
}

pub struct StateC {
    final_value: usize,
}
