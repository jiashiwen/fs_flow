use clap::{Arg, Command};

pub fn new_task_cmd() -> Command {
    clap::Command::new("task")
        .subcommand(task_analyze_source())
        .subcommand(task_exec())
        .subcommand(task_list_source_objects())
}

fn task_exec() -> Command {
    clap::Command::new("exec")
        .about("execute task description yaml file")
        .args(&[Arg::new("filepath")
            .value_name("filepath")
            .required(true)
            .index(1)
            .help("execute task description yaml file")])
}

fn task_analyze_source() -> Command {
    clap::Command::new("analyze")
        .about("analyze source objects destributed")
        .args(&[Arg::new("filepath")
            .value_name("filepath")
            .required(true)
            .index(1)
            .help("analyze source objects destributed")])
}

fn task_list_source_objects() -> Command {
    clap::Command::new("list_objects")
        .about("list source objects to files")
        .args(&[Arg::new("taskfile")
            .value_name("taskfile")
            .required(true)
            .index(1)
            .help("list source objects to file")])
        .args(&[Arg::new("list_files_folder")
            .value_name("folder")
            .required(true)
            .index(2)
            .help("folder for list files")])
}
