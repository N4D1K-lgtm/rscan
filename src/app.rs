use color_eyre::Result;
use crossterm::event::{KeyCode::Char, KeyEvent};
use ratatui::prelude::*;

use crate::{action::Action, message::Message, state::AppState, tui::Tui, ui::draw};

#[derive(Default)]
pub struct App {
  state: AppState,
}

impl App {
  pub fn new() -> Self {
    Self::default()
  }

  pub async fn run(&mut self) -> Result<()> {
    let mut tui = Tui::new()?;

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
    match draw(f, &self.state) {
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
  fn handle_event(&mut self, evt: Message) -> Option<Action> {
    match evt {
      Message::Quit => Some(Action::Quit),
      Message::Tick => Some(Action::Tick),
      Message::Init => Some(Action::Render),
      Message::Render => Some(Action::Render),
      Message::Key(key) => map_key_event_to_action(key),

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
