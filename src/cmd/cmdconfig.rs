use clap::{arg, Arg, Command};

pub fn new_config_cmd() -> Command<'static> {
    clap::Command::new("config")
        .about("config")
        .subcommand(config_show())
        .subcommand(config_setting())
        .subcommand(config_save())
}

fn config_show() -> Command<'static> {
    clap::Command::new("show")
        .about("show some info ")
        .subcommand(config_show_default())
        .subcommand(config_show_current())
}

fn config_setting() -> Command<'static> {
    clap::Command::new("setting")
        .about("setting parameters")
        .subcommand(config_setting_server())
        .subcommand(config_setting_token())
}

fn config_save() -> Command<'static> {
    clap::Command::new("save")
        .about("save config")
        .subcommand(config_save_default())
        .subcommand(config_save_current())
}

fn config_save_default() -> Command<'static> {
    clap::Command::new("default")
        .about("save default config to file")
        .args(&[Arg::new("filepath").value_name("filepath").index(1)])
}

fn config_save_current() -> Command<'static> {
    clap::Command::new("current")
        .about("save current config to file")
        .args(&[Arg::new("filepath").value_name("filepath").index(1)])
}

fn config_show_default() -> Command<'static> {
    clap::Command::new("default").about("show config template")
}

fn config_show_current() -> Command<'static> {
    clap::Command::new("current").about("show current configuration ")
}

fn config_setting_server() -> Command<'static> {
    clap::Command::new("server")
        .about("setting server url")
        .arg(arg!(<addr> "redissyncer-server address 'http://127.0.0.1:8080'"))
}

fn config_setting_token() -> Command<'static> {
    clap::Command::new("token")
        .about("setting login token")
        .arg(arg!(<token_string> "setting redissyncer-server login token"))
}
