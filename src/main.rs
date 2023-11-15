use clap::Parser;
use cli::{command_exit::CommandExit, Cli};
use colored::Colorize;
use std::process::exit;
mod cli;

fn main() {
  let cli = Cli::parse();

  let result = match cli.command {
    Some(sub_commands) => match sub_commands {
      cli::SubCommands::SignIn(sign_in_args) => cli::auth::sign_in::run(sign_in_args),
      cli::SubCommands::SignOut => cli::auth::sign_out::run(),
      cli::SubCommands::SignUp(sign_up_args) => cli::auth::sign_up::run(sign_up_args),
      cli::SubCommands::Note => cli::note::get_notes::run(),
    },
    None => {
      unreachable!()
    }
  };

  result_exit(result);
}

fn result_exit(result: CommandExit) {
  match result {
    CommandExit::Normal(message) => println!("{}", message),
    CommandExit::Success(message) => println!("{}", message.green()),
    CommandExit::Error(message) => {
      println!("{}", format!("Error: {}", message).red());
      exit(1);
    }
  }
}
