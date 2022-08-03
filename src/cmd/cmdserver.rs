use clap::{arg, Command};

pub fn new_server_cmd() -> Command<'static> {
    clap::Command::new("server")
        .about("server")
        .subcommand(alive())
        .subcommand(setting())
}

pub fn alive() -> Command<'static> {
    clap::Command::new("alive").about("check redissyncer server alive")
}

pub fn setting() -> Command<'static> {
    clap::Command::new("setting")
        .about("set redissyner-server address")
        .arg(arg!(<addr> "redissyncer-server address 'http://127.0.0.1:8080'"))
}
