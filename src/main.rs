mod fsm;

fn build_state_machine() -> Result<fsm::StateMachine, fsm::Error> {
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

  Ok(fsm::StateMachine::new(setup))
}

fn mymain() -> Result<bool, fsm::Error> {
  let mut sm = build_state_machine()?;

  let fstate = sm.transition("starting")?;
  println!("Transitioned to {:?}", fstate);

  let fstate = sm.transition("ready")?;
  println!("Transitioned to {:?}", fstate);

  let fstate = sm.transition("stopping")?;
  println!("Transitioned to {:?}", fstate);

  let fstate = sm.transition("stopped")?;
  println!("Transitioned to {:?}", fstate);

  let fstate = sm.transition("error")?;
  println!("Transitioned to {:?}", fstate);

  Ok(true)
}

fn main() {
  match mymain() {
    Ok(_) => {
      println!("All OK");
    }
    Err(e) => {
      println!("{:?}", e);
    }
  }
}
