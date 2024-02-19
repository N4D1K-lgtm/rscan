use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Action {
  Tick,
  Quit,
  Render,
  None,
}
