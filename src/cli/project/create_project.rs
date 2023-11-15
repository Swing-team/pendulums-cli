use chrono::{DateTime, Utc};
use clap::Parser;
use colored::Colorize;
use reqwest::StatusCode;
use serde::Deserialize;

use crate::cli::command_exit::CommandExit;
use crate::cli::http_helper::HttpHelper;

#[derive(Debug, Parser)]
#[command(author = "Mohammad Rafigh", version, about, long_about = None)]
pub struct CreateProjectArgs {
  name: String,

  /// Email addresses separated by commas; e.g. john.doe@test.com,mary.doe@test.com
  #[arg(short, long, value_delimiter = ',')]
  admins: Vec<String>,

  /// Email addresses separated by commas; e.g. john.doe@test.com,mary.doe@test.com
  #[arg(short, long, value_delimiter = ',')]
  team_members: Vec<String>,

  /// 0: white, 1: salmon, 2: blue, 3: black, 4: yellow, 5: pink, 6: red, 7: green
  #[arg(short, long, default_value = "0")]
  color_palette: u8,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Project {
  // id: Option<String>,
  // name: String,
  // invitedUsers: Vect<String>;
  // activities: Activity[] = [];
  // owner: TeamMember;
  // teamMembers: Array<TeamMember>;
  // admins: Array<TeamMember>;
  // recentActivityName: string = null;
  // // [white == 0, salmon == 1, blue == 2, black == 3, yellow == 4, pink == 5, red == 6, green == 7]
  // colorPalette = 0;

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

pub fn run(create_project_args: CreateProjectArgs) -> CommandExit {
  // return get_notes();
  println!("{:?}", create_project_args);
  return CommandExit::Success(String::from("ewrw"));
}

// #[tokio::main]
// async fn get_notes() -> CommandExit {
//   let http_helper = HttpHelper::build();
//   let request = http_helper
//     .http_client
//     .get("https://app.pendulums.io/api/notes/getall");

//   let res = http_helper.request(request).await;
//   match res {
//     Ok(res) => match res.status() {
//       StatusCode::OK => {
//         return match res.json::<Vec<Note>>().await {
//           Ok(notes) => {
//             let mut result_string = String::new();
//             for note in notes {
//               result_string.push_str(&format!(
//                 "{}",
//                 "-----------------------------------------------------\n".cyan()
//               ));
//               result_string.push_str(&format!("{}: {}, ", "Title".yellow(), note.title));
//               result_string.push_str(&format!(
//                 "{}: {}\n",
//                 "Created at".yellow(),
//                 note.created_at.format("%Y-%m-%d")
//               ));
//               result_string.push_str(&format!("{}", note.content));
//               result_string.push_str(&format!("\n"));
//             }
//             CommandExit::Normal(String::from(result_string))
//           }
//           Err(_e) => {
//             println!("error is: {}", _e);
//             CommandExit::Error(String::from("Failed to get your notes"))
//           }
//         };
//       }
//       _ => CommandExit::Error(String::from("Failed to get notes")),
//     },
//     Err(command_exit) => command_exit,
//   }
// }
