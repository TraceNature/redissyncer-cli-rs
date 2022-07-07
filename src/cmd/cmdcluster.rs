use clap::{arg, Command};

pub fn new_cluster_cmd() -> Command<'static> {
    clap::Command::new("cluster")
        .about("cluster")
        .subcommand(cluster_task_cmd())
        .subcommand(cluster_node_cmd())
}

fn cluster_task_cmd() -> Command<'static> {
    clap::Command::new("task")
        .about("task operate")
        .subcommand(cluster_task_create())
        .subcommand(cluster_task_start())
        .subcommand(cluster_task_stop())
        .subcommand(cluster_task_remove())
        .subcommand(cluster_task_list())
}

fn cluster_task_create() -> Command<'static> {
    clap::Command::new("create")
        .about("create task")
        .arg(arg!(<path> "create task json file path"))
}

fn cluster_task_start() -> Command<'static> {
    clap::Command::new("start")
        .about("start task")
        .arg(arg!(<taskid> "input task id to stop"))
}

fn cluster_task_stop() -> Command<'static> {
    clap::Command::new("stop")
        .about("stop task")
        .arg(arg!(<taskid>  "input task id to stop"))
}

fn cluster_task_remove() -> Command<'static> {
    clap::Command::new("remove")
        .about("remove task")
        .arg(arg!(<taskid>  "input task id to stop"))
}

fn cluster_task_list() -> Command<'static> {
    clap::Command::new("list")
        .about("list tasks")
        .subcommand(cluster_task_list_all())
        .subcommand(cluster_task_list_by_ids())
        .subcommand(cluster_task_list_by_names())
        .subcommand(cluster_task_list_by_node())
}

fn cluster_task_list_all() -> Command<'static> {
    clap::Command::new("all")
        .about("list tasks by task ids")
        .arg(arg!([queryid] "input queryid if have"))
}

fn cluster_task_list_by_ids() -> Command<'static> {
    clap::Command::new("byid")
        .about("list tasks by task ids")
        .arg(arg!(<taskid> "input taskid"))
}

fn cluster_task_list_by_names() -> Command<'static> {
    clap::Command::new("bynames")
        .about("list tasks by task names")
        .arg(arg!(<tasksname>
            r"input tasks name if multi use ',' to splite"
        ))
}

fn cluster_task_list_by_node() -> Command<'static> {
    clap::Command::new("bynode")
        .about("list all tasks on node")
        .arg(arg!(<taskid> "input nodeId"))
}

fn cluster_node_cmd() -> Command<'static> {
    clap::Command::new("node")
        .about("node operate")
        .subcommand(cluster_node_all())
        .subcommand(cluster_node_inspect())
}

fn cluster_node_all() -> Command<'static> {
    clap::Command::new("all").about("show all node")
}

fn cluster_node_inspect() -> Command<'static> {
    clap::Command::new("inspect")
        .about("list node details by nodeID")
        .arg(arg!(<nodeid> "input nodeId"))
}
