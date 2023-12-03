use chrono::{DateTime, Utc, Local};
use colored::Colorize;
use reqwest::StatusCode;
use serde::Deserialize;

use crate::cli::{command_exit::CommandExit, get_environment};
use crate::cli::http_helper::HttpHelper;

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
  let http_helper = HttpHelper::build();
  let request = http_helper
    .http_client
    .get(get_environment().api_url + "/notes/getall");

  let res = http_helper.request(request).await;
  match res {
    Ok(res) => match res.status() {
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
                note.created_at.with_timezone(&Local).format("%Y-%m-%d")
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
      _ => CommandExit::Error(String::from("Failed to get notes")),
    },
    Err(command_exit) => command_exit,
  }
}
