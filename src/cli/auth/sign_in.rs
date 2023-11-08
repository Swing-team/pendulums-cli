use clap::Parser;

use regex::Regex;
use reqwest::StatusCode;
use rpassword::{self, read_password};
use serde::Deserialize;
use std::{collections::HashMap, process::exit};

const EMAIL_VERIFICATION: &str =
    r"^[a-zA-Z0-9+_%-+.]{1,256}@[a-zA-Z0-9][a-zA-Z0-9-]{0,64}(.[a-zA-Z0-9][a-zA-Z0-9-]{0,25})+$";

#[derive(Debug, Parser)]
#[command(author = "Armin Ghoreishi", version, about, long_about = None)]
pub struct SignIn {
    #[arg(short, long)]
    email: String,

    #[arg(short, long)]
    password: Option<String>,
}

pub fn run(mut sign_in_args: SignIn) {
    let email_regex = Regex::new(EMAIL_VERIFICATION).unwrap();

    if !email_regex.is_match(&sign_in_args.email) {
        panic!("Email format is not correct!");
    }

    if sign_in_args.password.is_none() {
        println!("Type your password");
        match read_password() {
            Ok(pass) => sign_in_args.password = Some(pass),
            Err(_) => {
                panic!("Test")
            }
        }
    }

    if sign_in_args.password.clone().unwrap().len() < 6
        || sign_in_args.password.clone().unwrap().len() > 32
    {
        panic!("Password length must be between 6 and 32 characters")
    }
    match sign_in(sign_in_args) {
        Ok(_) => {
            println!("Signin successful")
        }
        Err(_) => {
            exit(1);
        }
    };
}

#[tokio::main]
async fn sign_in(sign_in_args: SignIn) -> Result<(), reqwest::Error> {
    let mut sign_in = HashMap::new();
    sign_in.insert("email", sign_in_args.email);
    sign_in.insert("password", sign_in_args.password.clone().unwrap());

    let http_client = reqwest::Client::new();

    let res = match http_client
        .post("https://app.pendulums.io/api/auth/signin")
        .json(&sign_in)
        .send()
        .await
    {
        Ok(res) => res,
        Err(_e) => {
            panic!("No internet connection")
        }
    };
    println!("{}", res.status());
    match res.status() {
        StatusCode::OK => {
            match res.text().await {
                Ok(_) => {}
                Err(_e) => {
                    println!("{}", _e);
                    panic!("Failed to login")
                }
            };
        }
        StatusCode::BAD_REQUEST => {
            match res.json::<SignInBadRequest>().await {
                Ok(json) => panic!("{}", json.message),
                Err(_e) => {
                    panic!("Failed to login")
                }
            };
        }
        _ => panic!("Failed to login"),
    }
    Ok(())
}

#[derive(Deserialize)]
struct SignInBadRequest {
    #[allow(dead_code)]
    r#type: u32,
    message: String,
}
