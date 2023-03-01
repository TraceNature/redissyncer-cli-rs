use crate::cmd::cmdserver::new_server_cmd;
use crate::cmd::{new_cluster_cmd, new_config_cmd, new_task_cmd};
use crate::commons::CommandCompleter;
use crate::commons::SubCmd;

use crate::configure::{self, get_config, get_config_file_path, set_config, Config};
use crate::configure::{generate_default_config, set_config_file_path};
use crate::request::{req, ReqResult, Request, RequestLogin, RequestTaskListAll};
use crate::{configure::set_config_from_file, interact};
use clap::{Arg, ArgMatches, Command as clap_Command};
use lazy_static::lazy_static;
use log::{info, log};

use std::borrow::Borrow;
use std::{env, fs, thread};

use crate::cmd::cmdlogin::new_login_cmd;
use chrono::prelude::Local;
use serde_json::Value;
use std::fs::File;
use std::io::Read;
use std::process::Command;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::time::Duration;

pub static APP_NAME: &str = "redissyncer-cli";

lazy_static! {
    static ref CLIAPP: clap::Command<'static> = clap::Command::new(APP_NAME)
        .version("1.0")
        .author("Shiwen Jia. <jiashiwen@gmail.com>")
        .about("redissyncer command line interface")
        .arg_required_else_help(true)
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file")
                .takes_value(true)
        )
        .arg(
            Arg::new("interact")
                .short('i')
                .long("interact")
                .help("run as interact mod")
        )
        .arg(
            Arg::new("v")
                .short('v')
                .multiple_occurrences(true)
                .takes_value(true)
                .help("Sets the level of verbosity")
        )
        .subcommand(new_server_cmd())
        .subcommand(new_config_cmd())
        .subcommand(new_task_cmd())
        .subcommand(new_login_cmd())
        .subcommand(new_cluster_cmd());
    static ref SUBCMDS: Vec<SubCmd> = subcommands();
}

pub fn run_app() {
    let matches = CLIAPP.clone().get_matches();
    if let Some(c) = matches.value_of("config") {
        println!("config path is:{}", c);
        set_config_file_path(c.to_string());
    }
    cmd_match(&matches);
}

pub fn run_from(args: Vec<String>) {
    match clap_Command::try_get_matches_from(CLIAPP.to_owned(), args.clone()) {
        Ok(matches) => {
            cmd_match(&matches);
        }
        Err(err) => {
            err.print().expect("Error writing Error");
        }
    };
}

// 获取全部子命令，用于构建commandcompleter
pub fn all_subcommand(app: &clap_Command, beginlevel: usize, input: &mut Vec<SubCmd>) {
    let nextlevel = beginlevel + 1;
    let mut subcmds = vec![];
    for iterm in app.get_subcommands() {
        subcmds.push(iterm.get_name().to_string());
        if iterm.has_subcommands() {
            all_subcommand(iterm, nextlevel, input);
        } else {
            if beginlevel == 0 {
                all_subcommand(iterm, nextlevel, input);
            }
        }
    }
    let subcommand = SubCmd {
        level: beginlevel,
        command_name: app.get_name().to_string(),
        subcommands: subcmds,
    };
    input.push(subcommand);
}

pub fn get_command_completer() -> CommandCompleter {
    CommandCompleter::new(SUBCMDS.to_vec())
}

fn subcommands() -> Vec<SubCmd> {
    let mut subcmds = vec![];
    all_subcommand(CLIAPP.clone().borrow(), 0, &mut subcmds);
    subcmds
}

fn cmd_match(matches: &ArgMatches) {
    if let Some(c) = matches.value_of("config") {
        set_config_file_path(c.to_string());
        set_config_from_file(&get_config_file_path());
    } else {
        set_config_from_file("");
    }

    let config = get_config().unwrap();
    let server = config.server;
    let req = Request::new(server.clone()).unwrap();

    if matches.is_present("interact") {
        interact::run();
        return;
    }

    if let Some(ref matches) = matches.subcommand_matches("server") {
        if let Some(_) = matches.subcommand_matches("alive") {
            let rt = tokio::runtime::Runtime::new().unwrap();
            let async_req = async {
                // let result = req::get_baidu().await;
                let result = req.health().await;

                match result {
                    Ok(resp) => {
                        println!("{}", resp.status())
                    }
                    Err(e) => {
                        println!("{:?}", e);
                    }
                }
            };
            rt.block_on(async_req);
        };

        if let Some(addr) = matches.subcommand_matches("setting") {
            if let Some(server) = addr.value_of("addr") {
                let mut c = get_config();
                match c {
                    Ok(mut cfg) => {
                        cfg.server = server.to_string();
                        cfg.flush_to_file(get_config_file_path());
                        println!("set {} successful!", server);
                    }
                    Err(e) => {
                        eprintln!("{}", e);
                    }
                }
            }
        }
    }

    if let Some(ref login) = matches.subcommand_matches("login") {
        let u = login.value_of("username").expect("get username error!");
        let p = login.value_of("password").expect("get password error!");

        let rt = tokio::runtime::Runtime::new().unwrap();
        let async_req = async {
            let resp = req.login(u.to_string(), p.to_string()).await;
            // let result = ReqResult::new(resp);

            match resp {
                Ok(r) => match r.text().await {
                    Ok(t) => match serde_json::from_str::<Value>(t.as_str()) {
                        Ok(v) => {
                            if v["code"] != "2000" {
                                eprintln!("{}", t);
                                return;
                            }
                            let token = v["data"]["token"].as_str();
                            if let Some(t) = token {
                                // 显示token
                                println!("token: {}", t);
                                // token写入当前配置
                                let mut c = get_config().unwrap();
                                c.token = t.to_string();
                                c.flush_to_file(get_config_file_path());
                                println!("update your config file success!");
                                set_config(c);
                                println!("your current config update");

                                println!("{}", get_config_file_path())
                                // 刷新配置文件
                            }
                        }
                        Err(e) => {
                            println!("{:?}", e);
                        }
                    },
                    Err(e) => {
                        println!("{:?}", e);
                    }
                },
                Err(e) => {
                    println!("{:?}", e);
                }
            }
        };
        rt.block_on(async_req);
    }

    // 集群模式下的任务处理解析
    if let Some(ref cluster) = matches.subcommand_matches("cluster") {
        if let Some(ref task) = matches.subcommand_matches("task") {
            if let Some(ref create) = matches.subcommand_matches("create") {
                println!("create task!");
                // ToDo
                // 实现集群模式下的任务创建逻辑
            }

            if let Some(ref start) = matches.subcommand_matches("start") {
                println!("start task!")
                // ToDo
                // 实现集群模式下任务启动逻辑
            }

            if let Some(ref stop) = matches.subcommand_matches("stop") {
                println!("stop task!")
                // Todo
                // 实现集群模式下停止任务逻辑
            }

            if let Some(ref remove) = matches.subcommand_matches("remove") {
                println!("remove task!")
                // Todo
                // 实现集群模式下删除任务逻辑
            }

            if let Some(ref list) = matches.subcommand_matches("list") {
                match list.subcommand_name() {
                    Some("all") => {
                        let queryid = list.subcommand_matches("all").unwrap().value_of("queryid");
                        let mut module = RequestTaskListAll::default();
                        let rt = tokio::runtime::Runtime::new().unwrap();
                        let async_req = async {
                            match queryid {
                                None => {
                                    let resp = req.task_list_all(module).await;
                                    let result = ReqResult::new(resp);
                                    result.task_list_all_parsor().await;
                                }
                                Some(id) => {
                                    module.set_query_id(id.to_string());
                                    let resp = req.task_list_all(module).await;
                                    let result = ReqResult::new(resp);
                                    result.task_list_all_parsor().await;
                                }
                            }
                        };
                        rt.block_on(async_req);
                    }
                    Some("byid") => {
                        let queryid = list.subcommand_matches("byid").unwrap().value_of("taskid");
                        let rt = tokio::runtime::Runtime::new().unwrap();
                        let async_req = async {
                            let mut ids = vec![];
                            if let Some(id) = queryid {
                                ids.push(id.to_string());
                                let resp = req.task_list_by_ids(ids).await;
                                let result = ReqResult::new(resp);
                                result.normal_parsor().await;
                            }
                        };
                        rt.block_on(async_req);
                    }
                    Some("bynames") => {
                        let names = list
                            .subcommand_matches("bynames")
                            .unwrap()
                            .value_of("tasksname");
                        let rt = tokio::runtime::Runtime::new().unwrap();
                        let async_req = async {
                            // let mut namearry = names;
                            if let Some(namesstr) = names {
                                let namearry = namesstr.split(',').collect::<Vec<&str>>();
                                let resp = req.task_list_by_names(namearry).await;
                                let result = ReqResult::new(resp);
                                result.task_list_bynames_parsor().await;
                            }
                        };
                        rt.block_on(async_req);
                    }

                    _ => {}
                }
            }
        }

        if let Some(ref node) = matches.subcommand_matches("node") {}
    }

    // 直连连原生引擎的任务相关命令解析
    if let Some(ref matches) = matches.subcommand_matches("task") {
        if let Some(create) = matches.subcommand_matches("create") {
            let file = File::open(create.value_of("path").unwrap());
            match file {
                Ok(mut f) => {
                    let mut data = String::new();
                    if let Err(e) = f.read_to_string(&mut data) {
                        println!("{}", e);
                        return;
                    };
                    let rt = tokio::runtime::Runtime::new().unwrap();
                    let async_req = async {
                        // let resp = req.create_task(data).await;
                        let resp = req.origin_task_create(data).await;
                        let result = ReqResult::new(resp);
                        result.normal_parsor().await;
                    };
                    rt.block_on(async_req);
                }
                Err(e) => {
                    println!("{}", e);
                }
            }
        }

        if let Some(create) = matches.subcommand_matches("import") {
            let file = File::open(create.value_of("path").unwrap());
            match file {
                Ok(mut f) => {
                    let mut data = String::new();
                    if let Err(e) = f.read_to_string(&mut data) {
                        println!("{}", e);
                        return;
                    };
                    let rt = tokio::runtime::Runtime::new().unwrap();
                    let async_req = async {
                        let resp = req.origin_task_import(data).await;
                        let result = ReqResult::new(resp);
                        result.normal_parsor().await;
                    };
                    rt.block_on(async_req);
                }
                Err(e) => {
                    println!("{}", e);
                }
            }
        }

        if let Some(start) = matches.subcommand_matches("start") {
            if let Some(taskid) = start.value_of("taskid") {
                let rt = tokio::runtime::Runtime::new().unwrap();
                let async_req = async {
                    // let resp = req.task_start(taskid.to_string()).await;
                    let resp = req.origin_task_start(taskid.to_string()).await;
                    let result = ReqResult::new(resp);
                    result.normal_parsor().await;
                };
                rt.block_on(async_req);
            };
        }
        if let Some(stop) = matches.subcommand_matches("stop") {
            if let Some(taskid) = stop.value_of("taskid") {
                let rt = tokio::runtime::Runtime::new().unwrap();
                let async_req = async {
                    // let resp = req.task_stop(taskid.to_string()).await;
                    let resp = req.origin_task_stop(taskid.to_string()).await;
                    let result = ReqResult::new(resp);
                    result.normal_parsor().await;
                };
                rt.block_on(async_req);
            };
        }
        if let Some(remove) = matches.subcommand_matches("remove") {
            if let Some(taskid) = remove.value_of("taskid") {
                let rt = tokio::runtime::Runtime::new().unwrap();
                let async_req = async {
                    // let resp = req.task_remove(taskid.to_string()).await;
                    let resp = req.origin_task_remove(taskid.to_string()).await;
                    let result = ReqResult::new(resp);
                    result.normal_parsor().await;
                };
                rt.block_on(async_req);
            };
        }
        if let Some(list) = matches.subcommand_matches("list") {
            match list.subcommand_name() {
                Some("all") => {
                    let rt = tokio::runtime::Runtime::new().unwrap();
                    let async_req = async {
                        let resp = req.origin_task_list_all().await;
                        let result = ReqResult::new(resp);
                        result.origin_task_list_all_parsor().await;
                        // result.normal_parsor().await;
                    };
                    rt.block_on(async_req);
                    // let queryid = list.subcommand_matches("all").unwrap().value_of("queryid");
                    // let mut module = RequestTaskListAll::default();
                    // let rt = tokio::runtime::Runtime::new().unwrap();
                    // let async_req = async {
                    //     match queryid {
                    //         None => {
                    //             let resp = req.task_list_all(module).await;
                    //             let result = ReqResult::new(resp);
                    //             result.task_list_all_parsor().await;
                    //         }
                    //         Some(id) => {
                    //             module.set_query_id(id.to_string());
                    //             let resp = req.task_list_all(module).await;
                    //             let result = ReqResult::new(resp);
                    //             result.task_list_all_parsor().await;
                    //         }
                    //     }
                    // };
                    // rt.block_on(async_req);
                }
                Some("byid") => {
                    let queryid = list.subcommand_matches("byid").unwrap().value_of("taskid");
                    let rt = tokio::runtime::Runtime::new().unwrap();
                    let async_req = async {
                        let mut ids = vec![];
                        if let Some(id) = queryid {
                            ids.push(id.to_string());
                            // let resp = req.task_list_by_ids(ids).await;
                            let resp = req.origin_task_list_by_id(ids).await;
                            let result = ReqResult::new(resp);
                            result.normal_parsor().await;
                        }
                    };
                    rt.block_on(async_req);
                }
                Some("bynames") => {
                    let names = list
                        .subcommand_matches("bynames")
                        .unwrap()
                        .value_of("tasksname");
                    let rt = tokio::runtime::Runtime::new().unwrap();
                    let async_req = async {
                        // let mut namearry = names;
                        if let Some(namesstr) = names {
                            let namearry = namesstr.split(',').collect::<Vec<&str>>();

                            let resp = req.task_list_by_names(namearry).await;
                            let result = ReqResult::new(resp);
                            result.task_list_bynames_parsor().await;
                        }
                    };
                    rt.block_on(async_req);
                }

                _ => {}
            }
        }
    }

    if let Some(config) = matches.subcommand_matches("config") {
        if let Some(show) = config.subcommand_matches("show") {
            match show.subcommand_name() {
                Some("current") => {
                    let current = configure::get_config().expect("get current configure error!");
                    let yml =
                        serde_yaml::to_string(&current).expect("pars configure to yaml error!");
                    println!("{}", yml);
                }
                Some("default") => {
                    let config = Config::default();
                    let yml = serde_yaml::to_string(&config);
                    match yml {
                        Ok(y) => {
                            println!("{}", y);
                        }
                        Err(e) => {
                            log::error!("{}", e);
                        }
                    }
                }
                _ => {}
            }
        }
        if let Some(gen_config) = config.subcommand_matches("gendefault") {
            let mut file = String::from("");
            if let Some(path) = gen_config.value_of("filepath") {
                file.push_str(path);
            } else {
                file.push_str("config_default.yml")
            }
            if let Err(e) = generate_default_config(file.as_str()) {
                log::error!("{}", e);
                return;
            };
            println!("{} created!", file);
        }
    }
}
