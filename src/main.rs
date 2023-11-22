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
      cli::SubCommands::Project(command) => match command.sub_command {
        Some(command) => {
          match command {
            cli::project::ProjectSubCommands::Create(create_project_args) => cli::project::create_project::run(create_project_args),
            cli::project::ProjectSubCommands::List(list_projects_args) => cli::project::list_projects::run(list_projects_args),
          }
        },
        None => {
          unreachable!()
        }
      },
      cli::SubCommands::Activity(command) => match command.sub_command {
        Some(command) => match command {
          cli::activity::ActivitySubCommands::Start(start_activity_args) => {
            cli::activity::start_activity::run(start_activity_args)
          }
        },
        None => {
          unreachable!()
        }
      },
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
