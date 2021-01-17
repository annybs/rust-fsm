// ANY state is a wildcard for any user-defined state (i.e. excluding INITIAL)
pub const ANY: &str = "*";

// a new StateMachine begins in INITIAL state
pub const INITIAL: &str = "";

#[derive(Debug)]
pub enum Error {
  InvalidStateID,
  NoTransition,
  NoState,
}

#[derive(Debug)]
pub struct FiniteState {
  pub id: String,
}

pub struct State {
  pub id: String,
}

pub struct StateMachine {
  current_state: FiniteState,
  // started: bool,
  states: Vec<State>,
  transitions: Vec<Transition>,
}

impl StateMachine {
  pub fn new() -> StateMachine {
    StateMachine{
      current_state: FiniteState {
        id: String::from(INITIAL),
      },
      // started: false,
      states: Vec::<State>::new(),
      transitions: Vec::<Transition>::new(),
    }
  }

  // add a new state
  pub fn add_state(&mut self, id: &str) -> Result<&State, Error> {
    if id == ANY || id == INITIAL {
      Err(Error::InvalidStateID)
    } else {
      self.states.push(State {
        id: String::from(id),
      });
      Ok(&self.states[self.states.len()-1])
    }
  }

  // add a new transition
  pub fn add_transition(&mut self, prev: &str, next: &str) -> Result<&Transition, Error> {
    if next == ANY || next == INITIAL {
      Err(Error::InvalidStateID)
    } else {
      self.transitions.push((String::from(prev), String::from(next)));
      Ok(&self.transitions[self.transitions.len()-1])
    }
  }

  // create a new local state (similar to a state with instance metdata)
  fn create_local_state(&self, state: &State) -> FiniteState {
    FiniteState {
      id: state.id.to_string(),
    }
  }

  // get a state by id
  pub fn get_state(&self, id: &str) -> Option<&State> {
    let mut state = None;
    for s in self.states.iter() {
      if id == &s.id {
        state = Some(s);
        break;
      }
    }
    state
  }

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

  // get all transitions from given previous state to any other, including ANY previous state
  fn get_transitions_from(&self, prev: &str) -> Option<Vec<Transition>> {
    let mut transitions = Vec::new();
    for (p, n) in self.transitions.iter() {
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

  // pub fn start(&mut self) {}

  // transition from current state to given next state. error if transition or next state does not exist
  pub fn transition(&mut self, next: &str) -> Result<&FiniteState, Error> {
    match self.get_transition(&self.current_state.id, next) {
      None => {
        Err(Error::NoTransition)
      }
      Some(_transition) => {
        match self.get_state(next) {
          None => {
            Err(Error::NoState)
          }
          Some(state) => {
            self.current_state = self.create_local_state(state);
            Ok(&self.current_state)
          }
        }
      }
    }
  }
}

// A Transition is a tuple of (prev id, next id)
pub type Transition = (String, String);
