pub mod start_activity;
pub mod get_current_activity;
pub mod stop_current_activity;
pub mod delete_activity;

use clap::{Parser, Subcommand};
use start_activity::StartActivityArgs;

use self::delete_activity::DeleteActivityArgs;

#[derive(Debug, Parser)]
#[command(author = "Armin Ghoreishi", version, about, arg_required_else_help = true)]
pub struct ActivityCommand {
  #[command(subcommand)]
  pub sub_command: Option<ActivitySubCommands>,
}

#[derive(Debug, Subcommand)]
pub enum ActivitySubCommands {
  /// Start new activity
  #[command(name = "start")]
  Start(StartActivityArgs),

  /// Get current activity
  #[command(name = "current")]
  GetCurrentActivity,

  #[command(name = "stop")]
  StopCurrentActivity,

  #[command(name = "delete")]
  DeleteActivity(DeleteActivityArgs)
}