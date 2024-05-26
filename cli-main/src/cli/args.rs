use clap::{command, Parser};

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Args {
    /// Login-User-Name of person who wants to book a class
    #[arg(short, long)]
    pub username: String,

    /// User-Password of the person who wants to book a class
    #[arg(short, long)]
    pub password: String,

    /// Optional Flag to signal that we want the following course without prompting the user
    #[arg(short, long)]
    pub course_name: Option<String>,
}
