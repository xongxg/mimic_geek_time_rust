use async_trait::async_trait;
use log::debug;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // pretty_env_logger::init();
    let states = &[State::State1, State::State2, State::State3];

    let req: Request = Default::default();
    let mut res: Response = Default::default();
    let mut machine = StateMachine::new(states);

    while let Some((state, mut transition)) = machine.next() {
        debug!("Current state {:?}", state);
        let next_state = transition.next(&req, &mut res).await?;

        if let Some(state) = next_state {
            debug!("Advance state {:?}", state);
            machine.advance(state);
        } else {
            debug!("State machine completed");
            machine.stop();
        }
    }

    Ok(())
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum State {
    State1,
    State2,
    State3,
}

#[async_trait]
pub trait Transition {
    async fn next(
        &mut self,
        request: &Request,
        response: &mut Response,
    ) -> Result<Option<State>, Box<dyn std::error::Error>>;
}

/// Mock configuration for the state machine.
#[derive(Debug, Default)]
pub struct Request {
    pub some_config: bool,
}

/// Mock response object that can capture intermediary state
/// to be passed to future transitions.
#[derive(Debug, Default)]
pub struct Response {
    pub some_data: usize,
}

#[derive(Debug)]
pub struct StateMachine<'a> {
    states: &'a [State],
    index: usize,
}

impl<'a> StateMachine<'a> {
    pub fn new(states: &'a [State]) -> Self {
        Self { states, index: 0 }
    }

    pub fn advance(&mut self, state: State) {
        let index = self.states.iter().position(|s| s == &state);
        if let Some(index) = index {
            self.index = index;
        } else {
            self.stop();
        }
    }

    pub fn stop(&mut self) {
        self.index = self.states.len();
    }
}

impl<'a> Iterator for StateMachine<'a> {
    type Item = (State, Box<dyn Transition>);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(state) = self.states.get(self.index) {
            let transition: Box<dyn Transition> = match state {
                State::State1 => Box::new(State1 {}),
                State::State2 => Box::new(State2 {}),
                State::State3 => Box::new(State3 {}),
            };

            Some((state.clone(), transition))
        } else {
            None
        }
    }
}

struct State1;

#[async_trait]
impl Transition for State1 {
    async fn next(
        &mut self,
        request: &Request,
        response: &mut Response,
    ) -> Result<Option<State>, Box<dyn Error>> {
        if request.some_config {
            Ok(Some(State::State3))
        } else {
            response.some_data = 10;
            Ok(Some(State::State2))
        }
    }
}

struct State2;

#[async_trait]
impl Transition for State2 {
    async fn next(
        &mut self,
        request: &Request,
        response: &mut Response,
    ) -> Result<Option<State>, Box<dyn Error>> {
        debug!("State 2 got data {}", response.some_data);
        Ok(Some(State::State3))
    }
}
struct State3;

#[async_trait]
impl Transition for State3 {
    async fn next(
        &mut self,
        request: &Request,
        response: &mut Response,
    ) -> Result<Option<State>, Box<dyn Error>> {
        Ok(None)
    }
}
