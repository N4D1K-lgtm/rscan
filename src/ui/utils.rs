use ratatui::prelude::*;

pub fn centered_rect(r: Rect, percent_x: u16, percent_y: u16) -> Rect {
  let v = vertical_center(r, percent_y);
  horizontal_center(v, percent_x)
}

pub fn horizontal_center(r: Rect, percent_x: u16) -> Rect {
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

pub fn vertical_center(r: Rect, percent_y: u16) -> Rect {
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
