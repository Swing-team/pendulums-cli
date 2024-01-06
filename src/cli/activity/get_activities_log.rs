use inquire::{InquireError, Select};
use reqwest::StatusCode;
use serde::Deserialize;

use crate::cli::{
  command_exit::CommandExit,
  get_environment,
  http_helper::HttpHelper,
  project::{list_projects::list_projects, Project},
};

use super::activity_helper::print_activities;

#[derive(Debug, Clone, Deserialize)]
pub struct Activity {
  pub id: String,
  pub name: String,
  #[serde(alias = "startedAt")]
  pub started_at: String,
  #[serde(alias = "stoppedAt")]
  pub stopped_at: String
}

pub fn run() -> CommandExit {
  // armin: I added these sapces to place the text below next to the spiner
  print!("         Fetching your projects...");
  let projects = list_projects();
  match projects {
    Ok(projects) => {
      let ans: Result<Project, InquireError> =
        Select::new("Which project are you working on?", projects).prompt();

      match ans {
        Ok(selected_project) => get_project_activities(selected_project.id.unwrap()),
        Err(_) => CommandExit::Error(String::from("You need to choose a project")),
      }
    }
    Err(command_exit) => command_exit,
  }
}

#[tokio::main]
pub async fn get_project_activities(project_id: String) -> CommandExit {
  let http_helper = HttpHelper::build();
  let request = http_helper.http_client.get(String::from(
    get_environment().api_url + "/projects/" + &project_id + "/activities",
  ));

  let res = http_helper.request(request).await;
  match res {
    Ok(res) => match res.status() {
      StatusCode::OK => match res.json::<Vec<Activity>>().await {
        Ok(activities) => {
          print_activities(activities)
        },
        Err(_e) => {
          println!("{:?}", _e);
          CommandExit::Error(String::from("Failed to get current activity!"))
        },
      },
      StatusCode::BAD_REQUEST => match res.text().await {
        Ok(message) => CommandExit::Error(String::from(message)),
        Err(_) => CommandExit::Error(String::from("Failed to get current activity!")),
      },
      StatusCode::NOT_FOUND => match res.text().await {
        Ok(message) => CommandExit::Error(String::from(message)),
        Err(_) => CommandExit::Error(String::from("Failed to get current activity!")),
      },
      _ => CommandExit::Error(String::from("Failed to get current activity!")),
    },
    Err(command_exit) => command_exit,
  }
}
