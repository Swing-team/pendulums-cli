use clap::Parser;
use reqwest::StatusCode;

use crate::cli::{command_exit::CommandExit, get_environment, http_helper::HttpHelper};

#[derive(Debug, Parser)]
pub struct DeleteActivityArgs {
  #[arg(short, long)]
  id: String,
}

pub fn run(delete_activity_args: DeleteActivityArgs) -> CommandExit {
  delete_activity(delete_activity_args.id)
}

#[tokio::main]
async fn delete_activity(activity_id: String) -> CommandExit {
  let http_helper = HttpHelper::build();
  let request = http_helper.http_client.delete(
    String::from(get_environment().api_url + "/activities/") + &activity_id,
  );

  let res = http_helper.request(request).await;
  match res {
    Ok(res) => match res.status() {
      StatusCode::NO_CONTENT => match res.text().await {
        Ok(_) => CommandExit::Success(String::from("Activity deleted")),
        Err(_e) => {
          CommandExit::Error(String::from("Failed to delete activity!"))
        }
      },
      StatusCode::BAD_REQUEST => match res.text().await {
        Ok(message) => CommandExit::Error(String::from(message)),
        Err(_e) => CommandExit::Error(String::from("Failed to delete activity!")),
      },
      _e => CommandExit::Error(String::from("Failed to delete activity!")),
    },
    Err(command_exit) => command_exit,
  }
}
