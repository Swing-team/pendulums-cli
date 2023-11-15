pub mod create_project;

use clap::{Parser, Subcommand};

use self::create_project::CreateProjectArgs;

#[derive(Debug, Parser)]
#[command(author, version, about, arg_required_else_help = true)]
pub struct ProjectCommand {
  #[command(subcommand)]
  pub sub_command: Option<ProjectSubCommands>,
}

#[derive(Debug, Subcommand)]
pub enum ProjectSubCommands {
  /// Create project sub command
  #[command(name = "create")]
  Create(CreateProjectArgs),
}
