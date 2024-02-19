use color_eyre::Result;
use ratatui::{
  prelude::*,
  widgets::{block::*, BorderType, Borders, Paragraph, Wrap},
};
use tui_nodes::*;

pub fn render_graph(mut f: Frame, rect: Rect) -> Result<()> {
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
