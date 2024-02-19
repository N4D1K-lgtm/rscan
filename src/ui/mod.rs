mod graph;
mod utils;
mod welcome;

use color_eyre::Result;
use ratatui::prelude::*;
use welcome::Welcome;

use crate::state::AppState;

pub fn draw(f: &mut Frame, _state: &AppState) -> Result<()> {
  let size = f.size();

  // Create an instance of the Welcome widget
  let welcome_widget = Welcome;

  // Render the Welcome widget
  f.render_widget(welcome_widget, f.size());

  Ok(())
}

