use clap::{arg, Command};

pub fn new_login_cmd() -> Command<'static> {
    clap::Command::new("login")
        .about("login")
        .arg(arg!(<username> "input username"))
        .arg(arg!(<password> "input password"))
}
