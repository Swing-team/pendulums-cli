use clap::Parser;

#[derive(Debug, Parser)]
#[command(author = "Armin Ghoreishi", version, about, long_about = None)]
pub struct UpdateActivityArgs {
  #[arg(short, long)]
  pub current: bool,
  #[arg(short, long)]
  pub name: String,
  #[arg(short, long)]
  pub id: Option<String>,
  #[arg(short = 's', long)]
  pub started_at: Option<String>,
  #[arg(short = 'S', long)]
  pub stopped_at: Option<String>,
}
