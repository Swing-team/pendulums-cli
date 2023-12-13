use clap::Parser;
use reqwest::StatusCode;

use super::{
  activity_helper::print_current_activity,
  get_current_activity::{get_current_activity, CurrentActivity},
};
use crate::cli::{command_exit::CommandExit, get_environment, http_helper::HttpHelper};
use serde::Serialize;

#[derive(Debug, Parser)]
#[command(author = "Armin Ghoreishi", version, about, long_about = None)]
pub struct UpdateCurrentActivityArgs {
  #[arg(short, long)]
  pub name: String,
}

#[derive(Debug, Serialize)]
struct UpdateActivityDTO {
  id: String,
  name: String,
  #[serde(rename = "startedAt")]
  started_at: String,
}

#[derive(Debug, Serialize)]
struct UpdateCurrentActivityDTO {
  activity: UpdateActivityDTO,
}

pub fn run(update_args: UpdateCurrentActivityArgs) -> CommandExit {
  // FIXME: we should not retrieve current activity to access its id and projectId
  match get_current_activity() {
    Ok(current_activity) => {
      match update_current_actvity(
        UpdateCurrentActivityDTO {
          activity: UpdateActivityDTO {
            id: current_activity.id,
            started_at: current_activity.started_at,
            name: update_args.name,
          },
        },
        current_activity.project_id,
      ) {
        Ok(current_activity) => print_current_activity(current_activity),
        Err(e) => e,
      }
    }
    Err(e) => e,
  }
}

#[tokio::main]
async fn update_current_actvity(
  update_current_activity_dto: UpdateCurrentActivityDTO,
  project_id: String,
) -> Result<CurrentActivity, CommandExit> {
  let http_helper = HttpHelper::build();
  let request = http_helper
    .http_client
    .put(
      String::from(get_environment().api_url)
        + "/projects/"
        + &project_id
        + "/activities/current/"
        + &update_current_activity_dto.activity.id,
    )
    .json(&update_current_activity_dto);

  let res = http_helper.request(request).await;
  match res {
    Ok(res) => match res.status() {
      StatusCode::OK => match res.json::<CurrentActivity>().await {
        Ok(activity) => Ok(activity),
        Err(_e) => Err(CommandExit::Error(String::from(
          "Failed to update current activity",
        ))),
      },
      StatusCode::BAD_REQUEST => match res.text().await {
        Ok(message) => Err(CommandExit::Error(String::from(message))),
        Err(_e) => Err(CommandExit::Error(String::from(
          "Failed to update current activity",
        ))),
      },
      _e => Err(CommandExit::Error(String::from(
        "Failed to update current activity",
      ))),
    },
    Err(_) => Err(CommandExit::Error(String::from(
      "Failed to update current activity sfsdfasdf",
    ))),
  }
}
