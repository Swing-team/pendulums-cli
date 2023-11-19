pub mod create_project;
pub mod list_projects;

use clap::{Parser, Subcommand};
use create_project::CreateProjectArgs;
use serde::Deserialize;

use self::list_projects::ListProjectsArgs;

#[derive(Debug, Parser)]
#[command(author, version, about, arg_required_else_help = true)]
pub struct ProjectCommand {
  #[command(subcommand)]
  pub sub_command: Option<ProjectSubCommands>,
}

#[derive(Debug, Subcommand)]
pub enum ProjectSubCommands {
  /// Create a new project
  #[command(name = "create")]
  Create(CreateProjectArgs),

  /// List projects
  #[command(name = "list")]
  List(ListProjectsArgs),
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Summary {
  user: UserSummary
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct UserSummary {
  projects: Vec<Project>
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Project {
  id: Option<String>,
  name: String,
  #[serde(alias = "invitedUsers")]
  invited_users: Vec<InvitedUser>,
  owner: TeamMember,
  #[serde(alias = "teamMembers")]
  team_members: Vec<TeamMember>,
  admins: Vec<TeamMember>,
  #[serde(alias = "recentActivityName")]
  recent_activity_name: Option<String>,
  #[serde(alias = "colorPalette")]
  color_palette: u8
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct TeamMember {
  id: Option<String>,
  email: String,
  name: Option<String>
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct InvitedUser {
  id: Option<String>,
  email: String,
  name: Option<String>,
  role: String
}