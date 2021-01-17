mod fsm;

use fsm::{FiniteState, Receiver, StateMachine};

struct MyStateMachine {}

impl Receiver for MyStateMachine {
  fn receive_state(&self, fstate: &FiniteState) {
    println!("Transitioned to {:?}", fstate);
  }
}

fn build_state_machine<'a> () -> Result<StateMachine<'a>, fsm::Error> {
  let mut setup = fsm::Setup::new();

  // error state can be entered at any time and is unrecoverable
  setup.add_state("error")?;
  setup.add_state("starting")?;
  setup.add_state("ready")?;
  setup.add_state("stopping")?;
  setup.add_state("stopped")?;

  setup.add_transition(fsm::INITIAL, "starting")?;
  setup.add_transition("starting", "ready")?;
  setup.add_transition("ready", "stopping")?;
  setup.add_transition("stopping", "stopped")?;
  setup.add_transition("stopped", "starting")?;
  setup.add_transition(fsm::ANY, "error")?;

  setup.set_receiver(&MyStateMachine{})?;
  Ok(fsm::StateMachine::new(setup))
}

fn test_fsm() -> Result<bool, fsm::Error> {
  let mut sm = build_state_machine()?;

  sm.transition("starting")?;
  sm.transition("ready")?;
  sm.transition("stopping")?;
  sm.transition("stopped")?;
  sm.transition("error")?;

  Ok(true)
}

fn main() {
  match test_fsm() {
    Ok(_) => {
      println!("All OK");
    }
    Err(e) => {
      println!("{:?}", e);
    }
  }
}
