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

    let cookie_store =
        reqwest_cookie_store::CookieStoreMutex::new(reqwest_cookie_store::CookieStore::new(None));
    let cookie_store = std::sync::Arc::new(cookie_store);

    let http_client = reqwest::Client::builder()
        .cookie_provider(cookie_store.clone())
        .build()
        .unwrap();

    let mut sp = PendulumsSpinner::start();
    let result = http_client
        .post("https://app.pendulums.io/api/auth/signin")
        .json(&sign_in)
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
                Ok(_) => {
                    // Write store back to disk
                    let mut writer = std::fs::File::create("cookies.json")
                        .map(std::io::BufWriter::new)
                        .unwrap();
                    let store = cookie_store.lock().unwrap();
                    store.save_json(&mut writer).unwrap();
                    CommandExit::Success(String::from("Sign in successful"))
                }
                Err(_e) => CommandExit::Error(String::from("Failed to sign in")),
            };
        }
        StatusCode::BAD_REQUEST => {
            return match res.json::<SignInBadRequest>().await {
                Ok(json) => CommandExit::Error(String::from(json.message)),
                Err(_e) => CommandExit::Error(String::from("Failed to sign in")),
            };
        }
        StatusCode::SERVICE_UNAVAILABLE => {
            return match res.text().await {
                Ok(_) => CommandExit::Success(String::from(
                    "You have reached the authentication limits, please try in a few minutes!",
                )),
                Err(_e) => CommandExit::Error(String::from("Failed to sign in")),
            };
        }
        _ => CommandExit::Error(String::from("Failed to sign in")),
    }
}

#[derive(Deserialize)]
struct SignInBadRequest {
    #[allow(dead_code)]
    r#type: u32,
    message: String,
}
