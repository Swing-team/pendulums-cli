use clap::{Parser, Subcommand};

pub mod auth;
pub mod spinner;
pub mod command_exit;
use auth::sign_in::SignIn;

#[derive(Debug, Parser)]
#[command(author, version, about,  arg_required_else_help=true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<SubCommands>,
}

#[derive(Debug, Subcommand)]
pub enum SubCommands {
    /// Sign in sub command
    #[command(name="signin")]
    SignIn(SignIn)
}