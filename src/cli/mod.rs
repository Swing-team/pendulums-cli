use clap::{Parser, Subcommand};

pub mod auth;
pub mod command_exit;
mod http_helper;
pub mod note;
pub mod spinner;
pub mod project;

use auth::sign_in::SignIn;
use auth::sign_up::SignUp;
use project::ProjectCommand;

#[derive(Debug, Parser)]
#[command(author, version, about, arg_required_else_help = true)]
pub struct Cli {
  #[command(subcommand)]
  pub command: Option<SubCommands>,
}

#[derive(Debug, Subcommand)]
pub enum SubCommands {
  /// Sign in sub command
  #[command(name = "signin")]
  SignIn(SignIn),

  /// Sign up sub command
  #[command(name = "signup")]
  SignUp(SignUp),

  /// Sign out sub command
  #[command(name = "signout")]
  SignOut,

  /// Gets all notes
  #[command(name = "notes")]
  Note,

  /// Project sub command
  #[command(name = "project")]
  Project(ProjectCommand),
}
