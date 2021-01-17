/*
constants
*/

// ANY state is a wildcard for any user-defined state (i.e. excluding INITIAL)
pub const ANY: &str = "*";

// a new StateMachine begins in INITIAL state
pub const INITIAL: &str = "";

/*
types
*/

#[derive(Debug)]
pub enum Error {
  InvalidStateID(String),
  NoTransition(String, String),
  NoState(String),
}

// a FiniteState represents an effective, living state (generally the current state of a StateMachine)
#[derive(Debug)]
pub struct FiniteState {
  pub id: String,
}

pub trait Receiver {
  fn receive_state(&self, fstate: &FiniteState);
}

// Setup for a StateMachine
pub struct Setup<'a> {
  receiver: Option<&'a dyn Receiver>,
  states: Vec<State>,
  transitions: Vec<Transition>,
}

// a State represents a state definition
pub struct State {
  pub id: String,
}

// de facto StateMachine
pub struct StateMachine<'a> {
  current_state: FiniteState,
  // TODO
  // started: bool,
  receiver: Option<&'a dyn Receiver>,
  states: Vec<State>,
  transitions: Vec<Transition>,
}

// a Transition is a tuple of (prev id, next id)
pub type Transition = (String, String);

/*
impl FiniteState
*/

impl FiniteState {
  // create a new FiniteState. state is not consumed
  pub fn new(state: &State) -> FiniteState {
    FiniteState {
      id: state.id.to_string(),
    }
  }
}

/*
impl Setup
*/

impl Setup<'_> {
  // add a new state
  pub fn add_state(&mut self, id: &str) -> Result<(), Error> {
    if id == ANY || id == INITIAL {
      Err(Error::InvalidStateID(String::from(id)))
    } else {
      self.states.push(State {
        id: String::from(id),
      });
      Ok(())
    }
  }

  // add a new transition
  pub fn add_transition(&mut self, prev: &str, next: &str) -> Result<(), Error> {
    if next == ANY || next == INITIAL {
      Err(Error::InvalidStateID(String::from(next)))
    } else {
      self.transitions.push((String::from(prev), String::from(next)));
      Ok(())
    }
  }

  // create a new Setup
  pub fn new<'a>() -> Setup<'a> {
    Setup{
      receiver: None,
      states: Vec::<State>::new(),
      transitions: Vec::<Transition>::new(),
    }
  }

  pub fn set_receiver(&mut self, receiver: &'static dyn Receiver) -> Result<(), Error> {
    self.receiver = Some(receiver);
    Ok(())
  }
}

impl WithStates for Setup<'_> {
  fn get_states(&self) -> &Vec<State> {
    &self.states
  }
}

impl WithTransitions for Setup<'_> {
  fn get_transitions(&self) -> &Vec<Transition> {
    &self.transitions
  }
}

/*
impl StateMachine
*/

impl StateMachine<'_> {
  // create a new StateMachine. setup is not consumed
  pub fn new<'a>(setup: Setup<'a>) -> StateMachine<'a> {
    let mut machine = StateMachine{
      current_state: FiniteState {
        id: String::from(INITIAL),
      },
      receiver: setup.receiver,
      // started: false,
      states: Vec::<State>::new(),
      transitions: Vec::<Transition>::new(),
    };
    for s in setup.get_states().iter() {
      machine.states.push(State {
        id: s.id.to_string(),
      });
    }
    for (p, n) in setup.get_transitions().iter() {
      machine.transitions.push((p.to_string(), n.to_string()));
    }
    machine
  }

  // TODO
  // pub fn start(&mut self) {}

  // transition from current state to given next state. error if transition or next state does not exist
  pub fn transition(&mut self, next: &str) -> Result<&FiniteState, Error> {
    match self.get_transition(&self.current_state.id, next) {
      None => {
        Err(Error::NoTransition(self.current_state.id.to_string(), String::from(next)))
      }
      Some(_transition) => {
        match self.get_state(next) {
          None => {
            Err(Error::NoState(String::from(next)))
          }
          Some(state) => {
            self.current_state = FiniteState::new(state);
            match self.receiver {
              None => {}
              Some(receiver) => {
                receiver.receive_state(&self.current_state);
              }
            }
            Ok(&self.current_state)
          }
        }
      }
    }
  }
}

impl WithStates for StateMachine<'_> {
  fn get_states(&self) -> &Vec<State> {
    &self.states
  }
}

impl WithTransitions for StateMachine<'_> {
  fn get_transitions(&self) -> &Vec<Transition> {
    &self.transitions
  }
}

/*
traits
*/

trait WithStates {
  // get a state by id
  fn get_state(&self, id: &str) -> Option<&State> {
    let mut state = None;
    for s in self.get_states().iter() {
      if id == &s.id {
        state = Some(s);
        break;
      }
    }
    state
  }

  // get own states
  fn get_states(&self) -> &Vec<State>;
}

trait WithTransitions {
  // get a transition from given previous state to given next state, including ANY previous state
  fn get_transition(&self, prev: &str, next: &str) -> Option<Transition> {
    let mut transition: Option<Transition> = None;
    let transitions = self.get_transitions_from(prev)?;
    for (p, n) in transitions.iter() {
      if n == next {
        transition = Some((p.to_owned(), n.to_owned()));
        break;
      }
    }
    transition
  }

  // get own transitions
  fn get_transitions(&self) -> &Vec<Transition>;

  // get all transitions from given previous state to any other, including ANY previous state
  fn get_transitions_from(&self, prev: &str) -> Option<Vec<Transition>> {
    let mut transitions = Vec::new();
    for (p, n) in self.get_transitions().iter() {
      let pstr = p.as_str();
      if prev == pstr || (prev != INITIAL && pstr == ANY) {
        transitions.push((p.to_owned(), n.to_owned()))
      }
    }
    if transitions.len() > 0 {
      Some(transitions)
    } else {
      None
    }
  }
}
