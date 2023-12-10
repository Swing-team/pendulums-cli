use chrono::Utc;
use clap::{command, Parser};
use inquire::{InquireError, Select};
use reqwest::StatusCode;
use serde::Serialize;

use crate::cli::http_helper::HttpHelper;
use crate::cli::project::list_projects::list_projects;
use crate::cli::project::Project;
use crate::cli::{command_exit::CommandExit, get_environment};

#[derive(Debug, Parser)]
#[command(author = "Armin Ghoreishi", version, about, long_about = None)]
pub struct StartActivityArgs {
  /// Activity name
  name: String,

  /// Project id
  project_id: Option<String>,
}

#[derive(Debug, Serialize)]
struct Activity {
  project: String,
  name: String,
  #[serde(rename = "startedAt")]
  started_at: String,
}

#[derive(Debug, Serialize)]
struct ActivityDTO {
  activity: Activity,
}

pub fn run(mut start_activity_args: StartActivityArgs) -> CommandExit {
  if start_activity_args.project_id.is_none() {
    // armin: I added these sapces to place the text below next to the spiner
    print!("         Fetching your projects...");
    let projects = list_projects();
    match projects {
      Ok(projects) => {
        let ans: Result<Project, InquireError> =
          Select::new("Which project are you working on?", projects).prompt();

        match ans {
          Ok(selected_project) => {
            start_activity_args.project_id = selected_project.id;
            start_activity(
              start_activity_args.project_id.unwrap(),
              start_activity_args.name,
            )
          }
          Err(_) => CommandExit::Error(String::from("You need to choose a project")),
        }
      }
      Err(command_exit) => command_exit,
    }
  } else {
    start_activity(
      start_activity_args.project_id.unwrap(),
      start_activity_args.name,
    )
  }
}

#[tokio::main]
async fn start_activity(project_id: String, activity_name: String) -> CommandExit {
  let new_activity = Activity {
    project: project_id,
    name: activity_name,
    started_at: Utc::now().timestamp_millis().to_string(),
  };
  let activity_dto = ActivityDTO {
    activity: new_activity,
  };

  let http_helper = HttpHelper::build();
  let request = http_helper
    .http_client
    .post(
      String::from(get_environment().api_url + "/projects/")
        + activity_dto.activity.project.as_str()
        + "/activities",
    )
    .json(&activity_dto);

  let res = http_helper.request(request).await;
  match res {
    Ok(res) => match res.status() {
      StatusCode::OK => match res.text().await {
        Ok(_) => CommandExit::Success(String::from("Activity started")),
        Err(_e) => {
          println!("error is: {}", _e);
          CommandExit::Error(String::from("Failed to start new activity!"))
        }
      },
      StatusCode::BAD_REQUEST => match res.text().await {
        Ok(message) => CommandExit::Error(String::from(message)),
        Err(_) => CommandExit::Error(String::from("Failed to start new activity!")),
      },
      _ => CommandExit::Error(String::from("Failed to start new activity!")),
    },
    Err(command_exit) => command_exit,
  }
}
