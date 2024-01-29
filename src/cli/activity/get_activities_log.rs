use clap::{command, Parser};
use colored::Colorize;
use inquire::{Confirm, InquireError, Select};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json;

use super::activity_helper::print_activities;
use crate::cli::{
  command_exit::CommandExit,
  get_environment,
  http_helper::HttpHelper,
  project::{list_projects::list_projects, Project},
};

#[derive(Debug, Parser)]
#[command(author = "Armin Ghoreishi", version, about, long_about = None)]
pub struct LogActivitiesArgs {
  #[arg(short, long)]
  pub all: bool,
  #[arg(short = 's', long)]
  pub started_date: Option<String>,
  #[arg(short = 'e', long)]
  pub end_date: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct LogActivitiesQueryParams {
  users: String,
  limit: u16,
  page: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Activity {
  pub id: String,
  pub name: String,
  #[serde(alias = "startedAt")]
  pub started_at: String,
  #[serde(alias = "stoppedAt")]
  pub stopped_at: String,
}

pub fn run(log_activities_args: LogActivitiesArgs) -> CommandExit {
  // armin: I added these sapces to place the text below next to the spiner
  print!("         Fetching your projects...");
  let projects = list_projects();
  match projects {
    Ok(projects) => {
      let limit: u16;
      let page: u32 = 0;
      if log_activities_args.all {
        // FIXME: We should do something in this case
        println!("{}", "WARNING: This is only 10000 of the latest activities per page, WE WILL FIX THIS LATER ".yellow());
        limit = 10000;
      } else {
        limit = 10;
      }

      let ans: Result<Project, InquireError> = Select::new("Select a project", projects).prompt();
      match ans {
        Ok(selected_project) => {
          let mut users: Vec<String> = Vec::new();
          for member in selected_project.team_members {
            users.push(member.id.unwrap());
          }
          let project_id = selected_project.id.unwrap();
          let query_params = LogActivitiesQueryParams {
            users: serde_json::to_string(&users).unwrap(),
            limit,
            page,
          };
          let activities_result = get_project_activities(&project_id, query_params.clone());
          return paginate_activities(
            project_id,
            query_params,
            activities_result,
            log_activities_args.all,
          );
        }
        Err(_) => CommandExit::Error(String::from("You need to choose a project")),
      }
    }
    Err(command_exit) => command_exit,
  }
}

pub fn paginate_activities(
  project_id: String,
  mut query_params: LogActivitiesQueryParams,
  activities_result: Result<Vec<Activity>, CommandExit>,
  all_arg: bool,
) -> CommandExit {
  match activities_result {
    Ok(activities) => {
      if activities.len() > 0 {
        print_activities(activities);
        let load_more = Confirm::new("Load more?").with_default(true).prompt();

        match load_more {
          Ok(true) => {
            if all_arg {
              query_params.page += 1000;
            } else {
              query_params.page += 1;
            }
            let activities_result = get_project_activities(&project_id, query_params.clone());
            return paginate_activities(project_id, query_params, activities_result, all_arg);
          }
          Ok(false) => {
            return CommandExit::Normal(String::from(""));
          }
          Err(_) => {
            return CommandExit::Error(String::from("Something went wront!"));
          }
        }
      } else {
        return CommandExit::Normal(String::from(""));
      }
    }
    Err(command_exit) => command_exit,
  }
}

#[tokio::main]
pub async fn get_project_activities(
  project_id: &String,
  query_params: LogActivitiesQueryParams,
) -> Result<Vec<Activity>, CommandExit> {
  let http_helper = HttpHelper::build();
  let request = http_helper
    .http_client
    .get(String::from(
      get_environment().api_url + "/projects/" + &project_id + "/activities",
    ))
    .query(&query_params);

  let res = http_helper.request(request).await;
  match res {
    Ok(res) => match res.status() {
      StatusCode::OK => match res.json::<Vec<Activity>>().await {
        Ok(activities) => Ok(activities),
        Err(_e) => {
          println!("error is : {:?}", _e);
          Err(CommandExit::Error(String::from(
            "Failed to get current activity!",
          )))
        }
      },
      StatusCode::BAD_REQUEST => match res.text().await {
        Ok(message) => Err(CommandExit::Error(String::from(message))),
        Err(_) => Err(CommandExit::Error(String::from(
          "Failed to get current activity!",
        ))),
      },
      StatusCode::NOT_FOUND => match res.text().await {
        Ok(message) => Err(CommandExit::Error(String::from(message))),
        Err(_) => Err(CommandExit::Error(String::from(
          "Failed to get current activity!",
        ))),
      },
      _e => {
        println!("error is : {:?}", _e);
        Err(CommandExit::Error(String::from(
          "Failed to get current activity!",
        )))
      }
    },
    Err(command_exit) => Err(command_exit),
  }
}
