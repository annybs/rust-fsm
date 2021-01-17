mod fsm;

fn mymain() -> Result<bool, fsm::Error> {
  let mut sm = fsm::StateMachine::new();

  // error state can be entered at any time and is unrecoverable
  sm.add_state("error")?;
  sm.add_state("starting")?;
  sm.add_state("ready")?;
  sm.add_state("stopping")?;
  sm.add_state("stopped")?;

  sm.add_transition(fsm::INITIAL, "starting")?;
  sm.add_transition("starting", "ready")?;
  sm.add_transition("ready", "stopping")?;
  sm.add_transition("stopping", "stopped")?;
  sm.add_transition("stopped", "starting")?;
  sm.add_transition(fsm::ANY, "error")?;

  let l = sm.transition("starting")?;
  println!("Transitioned to {:?}", l);

  let l = sm.transition("ready")?;
  println!("Transitioned to {:?}", l);

  let l = sm.transition("stopping")?;
  println!("Transitioned to {:?}", l);

  let l = sm.transition("stopped")?;
  println!("Transitioned to {:?}", l);

  let l = sm.transition("error")?;
  println!("Transitioned to {:?}", l);

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
