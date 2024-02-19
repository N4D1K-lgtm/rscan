use color_eyre::{eyre::Result, Report};
use ratatui::{
  layout::{Constraint, Direction, Layout},
  prelude::*,
  style::{Color, Modifier, Style},
  text::{Line, Span, Text},
  widgets::{block::*, BorderType, Borders, Paragraph, Wrap},
};
use tui_big_text::{BigTextBuilder, PixelSize};

pub struct Welcome;

impl Widget for Welcome {
  fn render(self, area: Rect, buf: &mut Buffer) {
    // Define the block with title and borders
    let block = Block::default()
      .borders(Borders::ALL)
      .title(Title::from("Welcome to RSCAN").alignment(Alignment::Center))
      .border_type(BorderType::Rounded)
      .style(Style::default().fg(Color::Gray));

    let inner_area = block.inner(area);

    // Render the block
    block.render(area, buf);

    // Define the layout for the big text and instructions
    let layout = Layout::default()
      .direction(Direction::Horizontal)
      .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
      .split(inner_area);

    // Create and render the big text
    let big_text = BigTextBuilder::default()
      .pixel_size(PixelSize::Full)
      .style(Style::default().fg(Color::Red))
      .lines(vec!["RSCAN".into(), "~~~~~".into()])
      .build()
      .unwrap(); // It's safe to unwrap in this context

    big_text.render(layout[0], buf);

    // Define the instructions
    let instructions_lines = vec![
      Line::from(Span::styled("Instructions:", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))),
      Line::from(Span::raw("Use arrow keys to navigate")),
      Line::from(Span::raw("Press 'Enter' to select")),
      Line::from(Span::raw("Press 'q' to quit")),
      Line::from(Span::styled("Author Notes:", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))),
      Line::from(Span::raw("Created by Kidan Nelson")),
      Line::from(Span::raw("For more information, visit https://github.com/N4D1K-lgtm/rscan")),
    ];

    let instructions_text = Text::from(instructions_lines);

    // Create and render the instructions paragraph
    let instructions = Paragraph::new(instructions_text)
      .style(Style::default().fg(Color::White))
      .alignment(Alignment::Left)
      .wrap(Wrap { trim: true });

    instructions.render(layout[1], buf);
  }
}

