use clap::Command;

pub fn new_server_cmd() -> Command<'static> {
    clap::Command::new("server")
        .about("server")
        .subcommand(alive())
}

pub fn alive() -> Command<'static> {
    clap::Command::new("alive").about("check redissyncer server alive")
}
