use clap::{Parser, Subcommand};

pub mod auth;
pub mod command_exit;
pub mod note;
pub mod spinner;

use auth::sign_in::SignIn;
use auth::sign_up::SignUp;

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

    /// Sign upn sub command
    #[command(name = "signup")]
    SignUp(SignUp),

    /// Gets all notes
    #[command(name = "note")]
    Note,
}
