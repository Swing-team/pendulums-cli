use clap::Parser;

mod cli;
use cli::Cli;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(sub_commands) => match sub_commands {
            cli::SubCommands::SignIn(sign_in_args) => {
                cli::auth::sign_in::run(sign_in_args);
            }
        },
        None => {
            println!("No subcommands....")
        }
    }
}
