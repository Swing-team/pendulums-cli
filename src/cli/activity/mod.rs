pub mod start_activity;

use clap::{Parser, Subcommand};
use start_activity::StartActivityArgs;

#[derive(Debug, Parser)]
#[command(author = "Armin Ghoreishi", version, about, arg_required_else_help = true)]
pub struct ActivityCommand {
  #[command(subcommand)]
  pub sub_command: Option<ActivitySubCommands>,
}

#[derive(Debug, Subcommand)]
pub enum ActivitySubCommands {
  /// Create a new project
  #[command(name = "start")]
  Start(StartActivityArgs),
}