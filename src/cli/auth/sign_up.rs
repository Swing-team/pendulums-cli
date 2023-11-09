use super::EMAIL_VERIFICATION;
use crate::cli::{command_exit::CommandExit, spinner::PendulumsSpinner};
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

    let http_client = reqwest::Client::new();

    let mut sp = PendulumsSpinner::start();
    let result = http_client
        .post("https://app.pendulums.io/api/auth/signup")
        .json(&sign_up)
        .send()
        .await;
    sp.stop();

    if result.is_err() {
        return CommandExit::Error(String::from("No internet connection!"));
    }

    let res = result.unwrap();
    match res.status() {
        StatusCode::OK => {
            return match res.text().await {
                Ok(_) => CommandExit::Success(String::from("Please checkout your email and confirm your email, then come back and sign in")),
                Err(_e) => CommandExit::Error(String::from("Failed to sign up")),
            };
        }
        StatusCode::BAD_REQUEST => {
            return match res.json::<SignUpBadRequest>().await {
                Ok(json) => CommandExit::Error(String::from(json.message)),
                Err(_e) => CommandExit::Error(String::from("Failed to sign up")),
            };
        }
        StatusCode::SERVICE_UNAVAILABLE => {
            return match res.text().await {
                Ok(_) => CommandExit::Success(String::from(
                    "You have reached the authentication limits, please try in a few minutes!",
                )),
                Err(_e) => CommandExit::Error(String::from("Failed to sign up")),
            };
        }
        _ => CommandExit::Error(String::from("Failed to sign up")),
    }
}

#[derive(Deserialize)]
struct SignUpBadRequest {
    #[allow(dead_code)]
    #[serde(alias = "errorCode")]
    error_code: u32,
    message: String,
}
