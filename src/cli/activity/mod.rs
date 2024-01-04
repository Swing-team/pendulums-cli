pub mod get_current_activity;
pub mod update_current_activity;
pub mod start_activity;
pub mod stop_current_activity;
pub mod activity_helper;
pub mod update_activity;

use clap::{Parser, Subcommand};
use start_activity::StartActivityArgs;

use self::update_activity::UpdateActivityArgs;

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
  /// Retrieve the status of current activity
  #[command(name = "status")]
  Status,

  /// Start new activity
  #[command(name = "start")]
  Start(StartActivityArgs),

  /// Update an activity
  #[command(name = "update")]
  Update(UpdateActivityArgs),

  #[command(name = "stop")]
  StopCurrentActivity,
}