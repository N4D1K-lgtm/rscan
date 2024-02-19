mod action;
mod app;
mod config;
mod event;
mod tui;
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
