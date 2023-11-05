
use clap::Parser;

#[derive(Parser)]
#[command(author = "Armin Ghoreishi", version, about, long_about = None)]
struct SignIn {
    /// Optional name to operate on
    email: String,

    password: String,
}
