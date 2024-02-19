mod action;
mod app;
mod cli;
mod config;
mod message;
mod state;

/// This is the main module for the TUI
pub mod tui;

mod ui;
mod utils;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
  config::initialize_logging()?;
  utils::initialize_panic_handler()?;

  let mut app = app::App::new();
  app.run().await?;

  Ok(())
}
