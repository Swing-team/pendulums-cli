use chrono::{DateTime, Days, Local};
use reqwest::StatusCode;
use serde::Serialize;

use crate::cli::{command_exit::CommandExit, get_environment, http_helper::HttpHelper};

use super::get_current_activity::get_current_activity;

#[derive(Debug, Clone, Serialize)]
struct ActivityDTO {
  #[serde(skip_serializing_if = "Option::is_none")]
  id: Option<String>,
  name: String,
  project: String,
  user: String,
  #[serde(rename = "startedAt")]
  started_at: String,
  #[serde(rename = "stoppedAt")]
  stopped_at: String,
}

#[derive(Debug, Clone, Serialize)]
struct UpdateActivityDTO {
  activity: ActivityDTO,
}

#[derive(Debug, Serialize)]
struct SyncActivitiesDTO {
  activities: Vec<ActivityDTO>,
}

pub fn run() -> CommandExit {
  match get_current_activity() {
    Ok(current_activity) => {
      let start_of_day_current_activity_started_at = DateTime::from_timestamp(
        current_activity.started_at.parse::<i64>().unwrap() / 1000,
        0,
      )
      .expect("Invalid Date")
      .date_naive()
      .and_hms_opt(0, 0, 0)
      .unwrap();

      let start_of_today = Local::now().date_naive().and_hms_opt(0, 0, 0).unwrap();
      let duration = start_of_today - start_of_day_current_activity_started_at;
      let difference_in_days = duration.num_days();

      if difference_in_days == 0 {
        // We don't need to split the activity
        return stop_current_activity(UpdateActivityDTO {
          activity: ActivityDTO {
            name: current_activity.name,
            started_at: current_activity.started_at,
            project: current_activity.project_id,
            id: Some(current_activity.id),
            stopped_at: Local::now().timestamp_millis().to_string(),
            user: current_activity.user,
          },
        });
      } else {
        // This is propbably a long running activity which is exceeded 24 hours so We need to split it into multiple activities
        let mut activities: Vec<ActivityDTO> = Vec::new();

        let update_current_activity_dto = ActivityDTO {
          name: current_activity.clone().name,
          started_at: current_activity.clone().started_at,
          id: Some(current_activity.clone().id),
          project: current_activity.clone().project_id,
          stopped_at: start_of_day_current_activity_started_at
            .date()
            .and_hms_opt(23, 59, 59)
            .unwrap()
            .and_local_timezone(Local)
            .unwrap()
            .timestamp_millis()
            .to_string(),
          user: current_activity.clone().user,
        };

        activities.push(update_current_activity_dto);

        for i in 1..difference_in_days {
          let next_day = start_of_day_current_activity_started_at
            .checked_add_days(Days::new(i.try_into().unwrap()))
            .unwrap();
          activities.push(ActivityDTO {
            name: current_activity.clone().name,
            started_at: next_day
              .and_local_timezone(Local)
              .unwrap()
              .timestamp_millis()
              .to_string(),
            project: current_activity.clone().project_id,
            stopped_at: next_day
              .date()
              .and_hms_opt(23, 59, 59)
              .unwrap()
              .and_local_timezone(Local)
              .unwrap()
              .timestamp_millis()
              .to_string(),
            id: None,
            user: current_activity.clone().user,
          });
        }

        activities.push(ActivityDTO {
          id: None,
          name: current_activity.clone().name,
          started_at: start_of_today
            .and_local_timezone(Local)
            .unwrap()
            .timestamp_millis()
            .to_string(),
          project: current_activity.clone().project_id,
          stopped_at: Local::now().timestamp_millis().to_string(),
          user: current_activity.clone().user,
        });

        return sync_old_activities(activities);
      }
    }
    Err(_) => CommandExit::Error(String::from("Failed to stop current activity!")),
  }
}

#[tokio::main]
async fn stop_current_activity(update_activity_dto: UpdateActivityDTO) -> CommandExit {
  let http_helper = HttpHelper::build();
  let request = http_helper
    .http_client
    .put(String::from(
      get_environment().api_url
        + "/projects/"
        + &update_activity_dto.clone().activity.project
        + "/activities/current/"
        + &update_activity_dto.clone().activity.id.unwrap(),
    ))
    .json(&update_activity_dto);

  let res = http_helper.request(request).await;
  match res {
    Ok(res) => match res.status() {
      StatusCode::OK => match res.text().await {
        Ok(_) => CommandExit::Success(String::from("Activity stopped")),
        Err(_e) => {
          println!("error is: {}", _e);
          CommandExit::Error(String::from("Faild to stop current activity!"))
        }
      },
      StatusCode::BAD_REQUEST => match res.text().await {
        Ok(message) => CommandExit::Error(String::from(message)),
        Err(_) => CommandExit::Error(String::from("Faild to stop current activity!")),
      },
      _ => CommandExit::Error(String::from("Faild to stop current activity!")),
    },
    Err(command_exit) => command_exit,
  }
}

#[tokio::main]
async fn sync_old_activities(activities: Vec<ActivityDTO>) -> CommandExit {
  // /sync/activities
  let http_helper = HttpHelper::build();

  let sync_activities_dto = SyncActivitiesDTO { activities };

  let request = http_helper
    .http_client
    .put(String::from(get_environment().api_url + "/sync/activities"))
    .json(&sync_activities_dto);

  let res = http_helper.request(request).await;
  match res {
    Ok(res) => match res.status() {
      StatusCode::OK => match res.text().await {
        Ok(e) => {
          println!("{:?}", e);
          CommandExit::Success(String::from("Activity stopped"))
        }
        Err(_e) => {
          println!("error is: {}", _e);
          CommandExit::Error(String::from("Faild to stop current activity!"))
        }
      },
      StatusCode::BAD_REQUEST => match res.text().await {
        Ok(message) => CommandExit::Error(String::from(message)),
        Err(_) => CommandExit::Error(String::from("Faild to stop current activity!")),
      },
      e => {
        println!("{:?}", e);
        CommandExit::Error(String::from("Faild to stop current activity!"))
      }
    },
    Err(command_exit) => command_exit,
  }
}
