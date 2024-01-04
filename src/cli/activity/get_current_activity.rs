use chrono::{DateTime, Local, Utc};
use reqwest::StatusCode;
use serde::Deserialize;

use crate::cli::{command_exit::CommandExit, get_environment, http_helper::HttpHelper};

use super::activity_helper::print_current_activity;

#[derive(Debug, Clone, Deserialize)]
pub struct CurrentActivity {
  pub id: String,
  #[serde(alias = "projectName")]
  pub project_name: String,
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
      print_current_activity(activity)
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
            "Failed to get current activity!",
          )))
        }
      },
      StatusCode::BAD_REQUEST => match res.text().await {
        Ok(message) => Err(CommandExit::Error(String::from(message))),
        Err(_) => Err(CommandExit::Error(String::from(
          "Failed to get current activity!",
        ))),
      },
      StatusCode::NOT_FOUND => {
        match res.text().await {
          Ok(message) => Err(CommandExit::Error(String::from(message))),
          Err(_) => Err(CommandExit::Error(String::from(
            "Failed to get current activity!",
          ))),
        }
      },
      _ => {
        Err(CommandExit::Error(String::from(
          "Failed to get current activity!",
        )))
      }
    },
    Err(command_exit) => Err(command_exit),
  }
}
