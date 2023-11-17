use super::EMAIL_VERIFICATION;
use crate::cli::http_helper::HttpHelper;
use crate::cli::{command_exit::CommandExit, API_URL};
use clap::Parser;
use regex::Regex;
use reqwest::StatusCode;
use rpassword::{self, read_password};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Parser)]
#[command(author = "Armin Ghoreishi", version, about, long_about = None)]
pub struct SignUp {
  #[arg(short, long)]
  email: String,

  #[arg(short, long)]
  password: Option<String>,
}

pub fn run(mut sign_up_args: SignUp) -> CommandExit {
  let email_regex = Regex::new(EMAIL_VERIFICATION).unwrap();

  if !email_regex.is_match(&sign_up_args.email) {
    return CommandExit::Error(String::from("Email format is not correct!"));
  }

  if sign_up_args.password.is_none() {
    println!("Type your password");
    match read_password() {
      Ok(pass) => sign_up_args.password = Some(pass),
      Err(_) => {}
    }
  }

  if sign_up_args.password.clone().unwrap().len() < 6
    || sign_up_args.password.clone().unwrap().len() > 32
  {
    return CommandExit::Error(String::from(
      "Password length must be between 6 and 32 characters!",
    ));
  }
  return sign_up(sign_up_args);
}

#[tokio::main]
async fn sign_up(sign_up_args: SignUp) -> CommandExit {
  let mut sign_up = HashMap::new();
  sign_up.insert("email", sign_up_args.email);
  sign_up.insert("password", sign_up_args.password.clone().unwrap());

  let http_helper = HttpHelper::build();
  let request = http_helper
    .http_client
    .post(API_URL.to_owned() + "/auth/signup")
    .json(&sign_up);

  let res = http_helper.request(request).await;
  match res {
    Ok(res) => match res.status() {
      StatusCode::OK => {
        return match res.text().await {
          Ok(_) => CommandExit::Success(String::from(
            "Please checkout your email and confirm your email, then come back and sign in",
          )),
          Err(_e) => CommandExit::Error(String::from("Failed to sign up")),
        };
      }
      StatusCode::BAD_REQUEST => {
        return match res.json::<SignUpBadRequest>().await {
          Ok(json) => CommandExit::Error(String::from(json.message)),
          Err(_e) => CommandExit::Error(String::from("Failed to sign up")),
        };
      }
      _ => CommandExit::Error(String::from("Failed to sign up")),
    },
    Err(command_exit) => command_exit,
  }
}

#[derive(Deserialize)]
struct SignUpBadRequest {
  #[allow(dead_code)]
  #[serde(alias = "errorCode")]
  error_code: u32,
  message: String,
}
