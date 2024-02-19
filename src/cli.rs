use clap::Parser;

/// rscan - A network enumeration tool/TUI by Kidan Nelson
#[derive(Parser, Debug)]
#[clap(author = "Kidan Nelson", version, about, long_about = None)]
pub struct Cli {
  /// Directory for configuration files
  #[clap(short, long, default_value_t = String::from("~/.config/rscantui"))]
  pub config_dir: String,

  /// Directory for log files
  #[clap(short, long, default_value_t = String::from("~/.local/share/rscantui/logs"))]
  pub log_dir: String,

  /// Directory for data files
  #[clap(short, long, default_value_t = String::from("~/.rscantui"))]
  pub data_dir: String,

  /// Frame rate for the TUI (frames per second)
  #[clap(short, long, default_value_t = 30)]
  pub framerate: u32,

  /// Tick rate for the TUI (ticks per second)
  #[clap(short = 'r', long, default_value_t = 10)]
  pub tick_rate: u32,
}
