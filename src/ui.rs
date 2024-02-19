use color_eyre::Result;
use ratatui::{
  layout::{Constraint, Direction, Layout},
  prelude::*,
  style::{Color, Modifier, Style},
  text::{Line, Span, Text},
  widgets::{block::*, BorderType, Borders, Paragraph, Wrap},
  Frame,
};
use tui_big_text::{BigTextBuilder, PixelSize};
use tui_nodes::*;

use crate::app::AppState;

#[derive(Default)]
pub struct Ui {}

impl Ui {
  pub fn new() -> Self {
    Self {}
  }

  pub fn draw(&self, f: &mut Frame, _state: &AppState) -> Result<()> {
    let rect = f.size();

    // let block = Block::default()
    //   .borders(Borders::ALL)
    //   .title(Title::from("Welcome to RSCAN").alignment(Alignment::Center))
    //   .border_type(BorderType::Rounded)
    //   .style(Style::default().fg(Color::Gray));
    //
    // let vcenter = vertical_center(rect, 80);
    //
    // let layout = Layout::default()
    //   .direction(Direction::Horizontal)
    //   .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
    //   .split(vcenter.inner(&Margin::new(8, 3)));
    //
    // let big_text = BigTextBuilder::default()
    //   .pixel_size(PixelSize::Full)
    //   .style(Style::default().fg(Color::Red))
    //   .lines(vec!["RSCAN".into(), "~~~~~".into()])
    //   .build()?;
    //
    // let big_text_area = layout[0];
    //
    // let instructions_lines = vec![
    //   Line::from(Span::styled("Instructions:", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))),
    //   Line::from(Span::raw("Use arrow keys to navigate")),
    //   Line::from(Span::raw("Press 'Enter' to select")),
    //   Line::from(Span::raw("Press 'q' to quit")),
    //   Line::from(Span::styled("Author Notes:", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))),
    //   Line::from(Span::raw("Created by Kidan Nelson")),
    //   Line::from(Span::raw("For more information, visit https://github.com/N4D1K-lgtm/rscan")),
    // ];
    //
    // let instructions_text = Text::from(instructions_lines);
    //
    // let instructions = Paragraph::new(instructions_text)
    //   .style(Style::default().fg(Color::White))
    //   .alignment(Alignment::Left)
    //   .wrap(Wrap { trim: true });
    //
    // let instructions_area = layout[1];

    let mut graph = NodeGraph::new(
      vec![
        NodeLayout::new((40, 8)).with_title("Router").with_border_type(BorderType::Thick),
        NodeLayout::new((20, 9)).with_title("Switch 1").with_border_type(BorderType::Thick),
        NodeLayout::new((30, 20)).with_title("Switch 2").with_border_type(BorderType::Thick),
        NodeLayout::new((20, 7)).with_title("Server 1").with_border_type(BorderType::Rounded),
        NodeLayout::new((20, 7)).with_title("Server 2").with_border_type(BorderType::Rounded),
      ],
      vec![
        Connection::new(0, 0, 1, 0).with_line_type(LineType::Rounded),
        Connection::new(0, 0, 2, 0).with_line_type(LineType::Plain),
        Connection::new(1, 0, 3, 0).with_line_type(LineType::Double),
        Connection::new(2, 0, 4, 0),
      ],
      rect.width as usize,
      rect.height as usize,
    );
    graph.calculate();
    let zones = graph.split(rect);

    let node_contents = vec!["IP: 192.168.1.1", "Ports: 24", "Ports: 48", "OS: Linux", "OS: Windows"];

    for (idx, ea_zone) in zones.into_iter().enumerate() {
      let content = Paragraph::new(node_contents[idx])
        .block(Block::default().borders(Borders::NONE))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });
      f.render_widget(content, ea_zone);
    }

    f.render_stateful_widget(graph, rect, &mut ());
    // f.render_widget(block, rect);
    // f.render_widget(big_text, big_text_area);
    // f.render_widget(instructions, instructions_area);
    //
    Ok(())
  }
}

fn centered_rect(r: Rect, percent_x: u16, percent_y: u16) -> Rect {
  let v = vertical_center(r, percent_y);
  horizontal_center(v, percent_x)
}

fn horizontal_center(r: Rect, percent_x: u16) -> Rect {
  let popup_layout = Layout::default()
    .direction(Direction::Horizontal)
    .constraints([
      Constraint::Percentage((100 - percent_x) / 2),
      Constraint::Percentage(percent_x),
      Constraint::Percentage((100 - percent_x) / 2),
    ])
    .split(r);
  popup_layout[1]
}

fn vertical_center(r: Rect, percent_y: u16) -> Rect {
  let popup_layout = Layout::default()
    .direction(Direction::Vertical)
    .constraints([
      Constraint::Percentage((100 - percent_y) / 2),
      Constraint::Percentage(percent_y),
      Constraint::Percentage((100 - percent_y) / 2),
    ])
    .split(r);
  popup_layout[1]
}

