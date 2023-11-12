use chrono::{DateTime, Utc};
use colored::Colorize;
use reqwest::StatusCode;
use serde::Deserialize;

use crate::cli::{command_exit::CommandExit, spinner::PendulumsSpinner};

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Note {
    #[serde(alias = "createdAt", with = "chrono::serde::ts_milliseconds")]
    created_at: DateTime<Utc>,
    #[serde(alias = "updatedAt")]
    updated_at: u64,
    id: String,
    title: String,
    content: String,
    #[serde(alias = "colorPalette")]
    color_palette: u8,
    #[serde(alias = "isArchived")]
    is_archived: Option<bool>,
    owner: String,
    project: Option<String>,
}

pub fn run() -> CommandExit {
    return get_notes();
}

#[tokio::main]
async fn get_notes() -> CommandExit {
    // Load an existing set of cookies, serialized as json
    let cookie_store = {
        if let Ok(file) = std::fs::File::open("cookies.json").map(std::io::BufReader::new) {
            // use re-exported version of `CookieStore` for crate compatibility
            reqwest_cookie_store::CookieStore::load_json(file).unwrap()
        } else {
            return CommandExit::Error(String::from("You need to sign in first!"));
        }
    };
    let cookie_store = reqwest_cookie_store::CookieStoreMutex::new(cookie_store);
    let cookie_store = std::sync::Arc::new(cookie_store);

    let mut sp = PendulumsSpinner::start();

    // Build a `reqwest` Client, providing the deserialized store
    let http_client = reqwest::Client::builder()
        .cookie_provider(std::sync::Arc::clone(&cookie_store))
        .build()
        .unwrap();

    let result = http_client
        .get("https://app.pendulums.io/api/notes/getall")
        .send()
        .await;
    sp.stop();

    if result.is_err() {
        return CommandExit::Error(String::from("No internet connection!"));
    }

    let res = result.unwrap();
    match res.status() {
        StatusCode::OK => {
            return match res.json::<Vec<Note>>().await {
                Ok(notes) => {
                    let mut result_string = String::new();
                    for note in notes {
                        result_string.push_str(&format!(
                            "{}",
                            "-----------------------------------------------------\n".cyan()
                        ));
                        result_string.push_str(&format!("{}: {}, ", "Title".yellow(), note.title));
                        result_string.push_str(&format!(
                            "{}: {}\n",
                            "Created at".yellow(),
                            note.created_at.format("%Y-%m-%d")
                        ));
                        result_string.push_str(&format!("{}", note.content));
                        result_string.push_str(&format!("\n"));
                    }
                    CommandExit::Normal(String::from(result_string))
                }
                Err(_e) => {
                    println!("error is: {}", _e);
                    CommandExit::Error(String::from("Failed to get your notes"))
                }
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
        StatusCode::FORBIDDEN => {
            return match res.text().await {
                Ok(_) => {
                    let _ = std::fs::remove_file("cookie.json");
                    CommandExit::Error(String::from("Please sign in first!"))
                }
                Err(_e) => CommandExit::Error(String::from("Failed to sign up")),
            };
        }
        e => {
            println!("{}", e);
            CommandExit::Error(String::from("Failed to get your notes"))
        }
    }
}
