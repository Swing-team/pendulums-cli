use chrono::{DateTime, Local, Utc};
use reqwest::StatusCode;
use serde::Deserialize;

use crate::cli::{command_exit::CommandExit, get_environment, http_helper::HttpHelper};

#[derive(Debug, Clone, Deserialize)]
pub struct CurrentActivity {
  pub id: String,
  #[serde(alias = "projectName")]
  project_name: String,
  #[serde(alias = "projectId")]
  pub project_id: String,
  pub name: String,
  #[serde(alias = "startedAt")]
  pub started_at: String,
  pub user: String
}

pub fn run() -> CommandExit {
  match get_current_activity() {
    Ok(activity) => {
      let started_at_seconds = activity.started_at.parse::<i64>().unwrap() / 1000;
      let started_at_date = DateTime::from_timestamp(started_at_seconds, 0).expect("Invalid Date");
      let now = Utc::now();
      let duration = now - started_at_date;
      println!(
        "{:<30} {:<30} {:<30} {:<30}",
        "Activity Name", "Started At", "Duration", "Project"
      ); // This prints the headers

      CommandExit::Normal(format!(
        "{:<30} {:<30} {:<30} {:<30}",
        activity.name,
        started_at_date
          .with_timezone(&Local)
          .format("%Y-%m-%d %H:%M:%S"),
        format!(
          "{}h {}m {}s",
          duration.num_hours(),
          duration.num_minutes() - (duration.num_hours() * 60),
          duration.num_seconds() - (duration.num_minutes() * 60)
        ),
        activity.project_name
      ))
    }
    Err(e) => e,
  }
}

#[tokio::main]
pub async fn get_current_activity() -> Result<CurrentActivity, CommandExit> {
  let http_helper = HttpHelper::build();
  let request = http_helper.http_client.get(String::from(
    get_environment().api_url + "/user/activities/current",
  ));

  let res = http_helper.request(request).await;
  match res {
    Ok(res) => match res.status() {
      StatusCode::OK => match res.json::<CurrentActivity>().await {
        Ok(activity) => Ok(activity),
        Err(_e) => {
          Err(CommandExit::Error(String::from(
            "Faild to get current activity!",
          )))
        }
      },
      StatusCode::BAD_REQUEST => match res.text().await {
        Ok(message) => Err(CommandExit::Error(String::from(message))),
        Err(_) => Err(CommandExit::Error(String::from(
          "Faild to get current activity!",
        ))),
      },
      StatusCode::NOT_FOUND => {
        match res.text().await {
          Ok(message) => Err(CommandExit::Error(String::from(message))),
          Err(_) => Err(CommandExit::Error(String::from(
            "Faild to get current activity!",
          ))),
        }
      },
      _ => {
        Err(CommandExit::Error(String::from(
          "Faild to get current activity!",
        )))
      }
    },
    Err(command_exit) => Err(command_exit),
  }
}
