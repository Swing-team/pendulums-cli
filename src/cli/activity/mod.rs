pub mod get_current_activity;
pub mod update_current_activity;
pub mod start_activity;
pub mod stop_current_activity;
pub mod activity_helper;

use clap::{Parser, Subcommand};
use start_activity::StartActivityArgs;

use self::update_current_activity::UpdateCurrentActivityArgs;

#[derive(Debug, Parser)]
#[command(
  author = "Armin Ghoreishi",
  version,
  about,
  arg_required_else_help = true
)]
pub struct ActivityCommand {
  #[command(subcommand)]
  pub sub_command: Option<ActivitySubCommands>,
}

#[derive(Debug, Subcommand)]
pub enum ActivitySubCommands {
  /// Start new activity
  #[command(name = "start")]
  Start(StartActivityArgs),

  /// Current activity subcommands
  #[command(subcommand, name="current")]
  CurrentActivity(CurrentActivitySubCommands),

  #[command(name = "stop")]
  StopCurrentActivity,
}

#[derive(Debug, Subcommand)]
pub enum CurrentActivitySubCommands {
  /// Retrieve the status of current activity
  #[command(name = "status")]
  Status,

  /// Update current activity
  #[command(name = "update")]
  Update(UpdateCurrentActivityArgs),
}
