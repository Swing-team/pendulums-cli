use colored::Colorize;
use reqwest::StatusCode;

use crate::cli::{command_exit::CommandExit, API_URL};
use crate::cli::http_helper::HttpHelper;

use super::{Project, Summary};

pub fn run() -> CommandExit {
  return match list_projects() {
    Ok(projects) => {
      println!(
        "{:<30} {:<30} {:<30} {:<30} {:<30} {:<30} {:<30}",
        "ID", "Name", "Invited Users", "Owner", "Team Members", "Admins", "Recent Activity Name"
      ); // This prints the headers

      for project in &projects {
        let invited_users = &project
          .invited_users
          .iter()
          .map(|i| String::from(&format!("{} <{}>", i.email, i.role)))
          .collect::<Vec<String>>()
          .join(", ");
        let team_members = &project
          .team_members
          .iter()
          .map(|t| String::from(&t.email))
          .collect::<Vec<String>>()
          .join(", ");
        let admins = &project
          .admins
          .iter()
          .map(|a| String::from(&a.email))
          .collect::<Vec<String>>()
          .join(", ");
        let owner = project.owner.name.clone().unwrap_or(project.owner.email.clone());
        println!(
          "{:<30} {:<30} {:<30} {:<30} {:<30} {:<30} {:<30}",
          project.id.clone().unwrap(),
          project.name,
          invited_users,
          owner,
          team_members,
          admins,
          project.recent_activity_name.clone().unwrap_or(String::from(""))
        ); // This prints the headers
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
    .get(API_URL.to_owned() + "/user/summary");

  let res = http_helper.request(request).await;
  match res {
    Ok(res) => match res.status() {
      StatusCode::OK => {
        return match res.json::<Summary>().await {
          Ok(summary) => Ok(summary.user.projects),
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
