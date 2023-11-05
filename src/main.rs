use clap::Parser;
use regex::Regex;
use reqwest::StatusCode;
use rpassword::{self, read_password};
use serde::Deserialize;
use std::collections::HashMap;

const EMAIL_VERIFICATION: &str =
    r"^[a-zA-Z0-9+_%-+.]{1,256}@[a-zA-Z0-9][a-zA-Z0-9-]{0,64}(.[a-zA-Z0-9][a-zA-Z0-9-]{0,25})+$";

#[derive(Parser)]
#[command(author = "Armin Ghoreishi", version, about, long_about = None)]
struct SignIn {
    #[arg(short, long)]
    email: String,

    #[arg(short, long)]
    password: Option<String>,
    #[arg(short, long)]
    x: Option<String>,
}

fn main() {
    let mut cli = SignIn::parse();
    let email_regex = Regex::new(EMAIL_VERIFICATION).unwrap();

    if !email_regex.is_match(&cli.email) {
        panic!("Email format is not correct!");
    }

    if cli.password.is_none() {
        println!("Type your password");
        match read_password() {
            Ok(pass) => cli.password = Some(pass),
            Err(_) => {
                panic!("Test")
            }
        }
    }

    if cli.password.clone().unwrap().len() < 6 || cli.password.clone().unwrap().len() > 32 {
        panic!("Password length must be between 6 and 32 characters")
    }
    signIn(cli);
}
#[tokio::main]
async fn signIn(user: SignIn) -> Result<(), reqwest::Error> {
    let mut sign_in = HashMap::new();
    sign_in.insert("email", user.email);
    sign_in.insert("password", user.password.clone().unwrap());

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
                Ok(txt) => {
                }
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
    r#type: u32,
    message: String,
}
