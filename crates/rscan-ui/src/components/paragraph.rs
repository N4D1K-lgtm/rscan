use ratatui::{
  prelude::*,
  widgets::{Block, Borders, Paragraph as RatatuiParagraph, Wrap},
};
use rscan_derive::{component, view};

use crate::View;

#[component]
pub fn Paragraph<'a>(
  text: impl Into<Text<'a>>,
  block: Option<Block<'a>>,
  style: Option<Style>,
  alignment: Option<Alignment>,
  wrap: Option<Wrap>,
) -> impl View {
  let paragraph = RatatuiParagraph::new(text)
    .block(block.unwrap_or_default())
    .style(style.unwrap_or_default())
    .alignment(alignment.unwrap_or(Alignment::Left))
    .wrap(wrap.unwrap_or(Wrap { trim: true }));

  view! { paragraph }
}
