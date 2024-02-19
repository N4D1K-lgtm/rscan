use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Action {
  Tick,
  Increment,
  Decrement,
  Quit,
  Render,
  None,
}
