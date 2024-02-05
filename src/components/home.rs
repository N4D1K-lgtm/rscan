use std::{collections::HashMap, time::Duration};

use color_eyre::eyre::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{prelude::*, widgets::*};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::UnboundedSender;

use super::{Component, Frame};
use crate::{
  action::Action,
  config::{Config, KeyBindings},
};

#[derive(Default)]
pub struct Home {
  command_tx: Option<UnboundedSender<Action>>,
  config: Config,
}

impl Home {
  pub fn new() -> Self {
    Self::default()
  }
}

impl Component for Home {
  fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
    self.command_tx = Some(tx);
    Ok(())
  }

  fn register_config_handler(&mut self, config: Config) -> Result<()> {
    self.config = config;
    Ok(())
  }

  fn update(&mut self, action: Action) -> Result<Option<Action>> {
    match action {
      Action::Tick => {},
      _ => {},
    }
    Ok(None)
  }

  fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> Result<()> {
    let rows = [Row::new(vec!["Cell1", "Cell2", "Cell3"])];
    // Columns widths are constrained in the same way as Layout...
    let widths = [Constraint::Length(5), Constraint::Length(5), Constraint::Length(10)];
    let table = Table::new(rows, widths)
    // ...and they can be separated by a fixed spacing.
    .column_spacing(1)
    // You can set the style of the entire Table.
    .style(Style::new().blue())
    // It has an optional header, which is simply a Row always visible at the top.
    .header(
        Row::new(vec!["Col1", "Col2", "Col3"])
            .style(Style::new().bold())
            // To add space between the header and the rest of the rows, specify the margin
            .bottom_margin(1),
    )
    // It has an optional footer, which is simply a Row always visible at the bottom.
    .footer(Row::new(vec!["Updated on Dec 28"]))
    // As any other widget, a Table can be wrapped in a Block.
    .block(Block::default().title("Table"))
    // The selected row and its content can also be styled.
    .highlight_style(Style::new().reversed())
    // ...and potentially show a symbol in front of the selection.
    .highlight_symbol(">>");

    f.render_widget(table, area);
    Ok(())
  }
}
