use crate::cli::API_URL;
use crate::cli::command_exit::CommandExit;
use crate::cli::http_helper::HttpHelper;
use reqwest::StatusCode;

pub fn run() -> CommandExit {
  return sign_out();
}

#[tokio::main]
async fn sign_out() -> CommandExit {
  let http_helper = HttpHelper::build();
  let request = http_helper
    .http_client
    .get(API_URL.to_owned() + "/auth/signout");

  let res = http_helper.request(request).await;
  return match res {
    Ok(res) => match res.status() {
      StatusCode::OK => {
        let _ = http_helper.remove_auth_cookie();
        CommandExit::Success(String::from("Sign out successful"))
      }
      e => {
        println!("{:?}", e);
        CommandExit::Error(String::from("Failed to sign out"))
      },
    },
    Err(command_exit) => command_exit,
  };
}
