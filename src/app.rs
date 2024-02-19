use color_eyre::Result;
use crossterm::event::{KeyCode::Char, KeyEvent};
use ratatui::{
  prelude::*,
  widgets::{Block, Borders},
};

use crate::{action::Action, event::Event, tui::Tui, ui::Ui};

#[derive(Default)]
pub struct AppState {
  should_quit: bool,
}

#[derive(Default)]
pub struct App {
  state: AppState,
  ui: Ui,
}

impl App {
  pub fn new() -> Self {
    Self::default()
  }

  pub async fn run(&mut self) -> Result<()> {
    let mut tui = Tui::new()?
            .tick_rate(4.0) // 4 ticks per second
            .frame_rate(30.0); // 30 frames per second

    tui.enter()?; // Starts event handler, enters raw mode, enters alternate screen

    loop {
      tui.draw(|f| {
        // Deref allows calling `tui.terminal.draw`
        self.ui(f);
      })?;

      if let Some(evt) = tui.next().await {
        // `tui.next().await` blocks till next event
        let mut maybe_action = self.handle_event(evt);
        while let Some(action) = maybe_action {
          maybe_action = self.update(action);
        }
      };

      if self.state.should_quit {
        break;
      }
    }

    tui.exit()?; // stops event handler, exits raw mode, exits alternate screen

    Ok(())
  }

  fn ui(&self, f: &mut Frame) {
    match self.ui.draw(f, &self.state) {
      Ok(_) => {},
      Err(_) => {},
    }
  }

  /// This is essentially a reducer. This purely matches actions to state changes.
  fn update(&mut self, action: Action) -> Option<Action> {
    match action {
      Action::Quit => self.state.should_quit = true,
      Action::Tick => {},
      Action::Render => {},
      _ => {},
    }
    None
  }

  // This maps events to actions.
  fn handle_event(&mut self, evt: Event) -> Option<Action> {
    match evt {
      Event::Quit => Some(Action::Quit),
      Event::Tick => Some(Action::Tick),
      Event::Init => Some(Action::Render),
      Event::Render => Some(Action::Render),
      Event::Key(key) => map_key_event_to_action(key),

      _ => None,
    }
  }
}

fn map_key_event_to_action(key: KeyEvent) -> Option<Action> {
  match key.code {
    Char('q') => Some(Action::Quit),
    _ => Some(Action::None),
  }
}
