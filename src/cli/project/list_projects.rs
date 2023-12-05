use clap::Parser;
use colored::Colorize;
use reqwest::StatusCode;

use crate::cli::{command_exit::CommandExit, get_environment};
use crate::cli::http_helper::HttpHelper;

use super::Project;

#[derive(Debug, Parser)]
#[command(author = "Mohammad Rafigh", version, about, long_about = None)]
pub struct ListProjectsArgs {
  #[arg(short, long)]
  admins: bool,

  #[arg(short, long)]
  team_members: bool,

  #[arg(short, long)]
  invited_users: bool,
}

pub fn run(list_projects_args: ListProjectsArgs) -> CommandExit {
  return match list_projects() {
    Ok(projects) => {
      let mut headers = String::from(format!(
        "{:<30} {:<30} {:<30} {:<30}",
        "ID", "Name", "Owner", "Recent Activity Name"
      ));
      if list_projects_args.admins {
        headers = headers + &format!(" {:<30}", "Admins");
      }
      if list_projects_args.team_members {
        headers = headers + &format!(" {:<30}", "Team Members");
      }
      if list_projects_args.invited_users {
        headers = headers + &format!(" {:<30}", "Invited Users");
      }

      println!("{}", headers);

      for project in &projects {
        let owner = project
          .owner
          .name
          .clone()
          .unwrap_or(project.owner.email.clone());
        let mut content = String::from(format!(
          "{:<30} {:<30} {:<30} {:<30}",
          project.id.clone().unwrap(),
          project.name,
          owner,
          project.recent_activity_name.clone().unwrap_or(String::from(""))
        ));

        if list_projects_args.admins {
          let admins = &project
            .admins
            .iter()
            .map(|a| String::from(&a.email))
            .collect::<Vec<String>>()
            .join(", ");

          content = content + &format!(" {:<30}", admins);
        }
        if list_projects_args.team_members {
          let team_members = &project
            .team_members
            .iter()
            .map(|t| String::from(&t.email))
            .collect::<Vec<String>>()
            .join(", ");

          content = content + &format!(" {:<30}", team_members);
        }
        if list_projects_args.invited_users {
          let invited_users = &project
            .invited_users
            .iter()
            .map(|i| String::from(&format!("{} <{}>", i.email, i.role)))
            .collect::<Vec<String>>()
            .join(", ");

          content = content + &format!(" {:<30}", invited_users);
        }

        match project.color_palette {
          0 => println!("{}", String::from(content).white()),
          1 => println!("{}", String::from(content).bright_red()),
          2 => println!("{}", String::from(content).cyan()),
          3 => println!("{}", String::from(content).black()),
          4 => println!("{}", String::from(content).yellow()),
          5 => println!("{}", String::from(content).bright_magenta()),
          6 => println!("{}", String::from(content).red()),
          7 => println!("{}", String::from(content).bright_green()),
          _ => println!("{}", String::from(content).white())
        }
      }
      CommandExit::Normal(String::from(""))
    }
    Err(command_exit) => command_exit,
  };
}

#[tokio::main]
pub async fn list_projects() -> Result<Vec<Project>, CommandExit> {
  let http_helper = HttpHelper::build();
  let request = http_helper
    .http_client
    .get(get_environment().api_url + "/projects");

  let res = http_helper.request(request).await;
  match res {
    Ok(res) => match res.status() {
      StatusCode::OK => {
        return match res.json::<Vec<Project>>().await {
          Ok(projects) => Ok(projects),
          Err(_e) => {
            println!("error is: {}", _e);
            Err(CommandExit::Error(String::from(
              "Failed to get your projects",
            )))
          }
        };
      }
      _ => Err(CommandExit::Error(String::from("Failed to get projects"))),
    },
    Err(command_exit) => Err(command_exit),
  }
}
