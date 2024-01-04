use chrono::{DateTime, Utc, Local};

use crate::cli::command_exit::CommandExit;

use super::get_current_activity::CurrentActivity;

pub fn print_current_activity(activity: CurrentActivity) -> CommandExit {
  let started_at_date =
    DateTime::from_timestamp(activity.started_at.parse::<i64>().unwrap() / 1000, 0)
      .expect("Invalid Date");
  let now = Utc::now();
  let duration = now - started_at_date;
  println!(
    "{:<30} {:<30} {:<30} {:<30}",
    "Activity Name", "Started At", "Duration", "Project"
  ); // This prints the headers

  CommandExit::Normal(format!(
    "{:<30} {:<30} {:<30} {:<30}",
    activity.name,
    started_at_date
      .with_timezone(&Local)
      .format("%Y-%m-%d %H:%M:%S"),
    format!(
      "{}h {}m {}s",
      duration.num_hours(),
      duration.num_minutes() - (duration.num_hours() * 60),
      duration.num_seconds() - (duration.num_minutes() * 60)
    ),
    activity.project_name
  ))
}
