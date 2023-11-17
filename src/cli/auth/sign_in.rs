use super::EMAIL_VERIFICATION;
use crate::cli::{command_exit::CommandExit, API_URL};
use crate::cli::http_helper::HttpHelper;
use clap::Parser;
use regex::Regex;
use reqwest::StatusCode;
use rpassword::{self, read_password};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Parser)]
#[command(author = "Armin Ghoreishi", version, about, long_about = None)]
pub struct SignIn {
  #[arg(short, long)]
  email: String,

  #[arg(short, long)]
  password: Option<String>,
}

pub fn run(mut sign_in_args: SignIn) -> CommandExit {
  let email_regex = Regex::new(EMAIL_VERIFICATION).unwrap();

  if !email_regex.is_match(&sign_in_args.email) {
    return CommandExit::Error(String::from("Email format is not correct!"));
  }

  if sign_in_args.password.is_none() {
    println!("Type your password");
    match read_password() {
      Ok(pass) => sign_in_args.password = Some(pass),
      Err(_) => {}
    }
  }

  if sign_in_args.password.clone().unwrap().len() < 6
    || sign_in_args.password.clone().unwrap().len() > 32
  {
    return CommandExit::Error(String::from(
      "Password length must be between 6 and 32 characters!",
    ));
  }

  return sign_in(sign_in_args);
}

#[tokio::main]
async fn sign_in(sign_in_args: SignIn) -> CommandExit {
  let mut sign_in = HashMap::new();
  sign_in.insert("email", sign_in_args.email);
  sign_in.insert("password", sign_in_args.password.clone().unwrap());

  let http_helper = HttpHelper::build();
  let request = http_helper
    .http_client
    .post(API_URL.to_owned() + "/auth/signin")
    .json(&sign_in);

  let res = http_helper.request(request).await;
  return match res {
    Ok(res) => match res.status() {
      StatusCode::OK => {
        http_helper.store_auth_cookie();
        CommandExit::Success(String::from("Sign in successful"))
      }
      StatusCode::BAD_REQUEST => match res.json::<SignInBadRequest>().await {
        Ok(json) => CommandExit::Error(String::from(json.message)),
        Err(_e) => CommandExit::Error(String::from("Failed to sign in")),
      },
      _ => CommandExit::Error(String::from("Failed to sign in")),
    },
    Err(command_exit) => command_exit,
  };
}

#[derive(Deserialize)]
struct SignInBadRequest {
  #[allow(dead_code)]
  r#type: u32,
  message: String,
}
