use std::path::Path;

use axis::models::{Job, Role, Task, Vars};

#[test]
fn generate() {
    let mut vars = Vars::new();
    vars.insert("key 1".to_string(), "val 1".to_string());
    vars.insert("key 2".to_string(), "val 2".to_string());
    vars.insert("key 3".to_string(), "val 3".to_string());

    let groups = vec!["group 1".to_string(), "group 2".to_string()];
    let hosts = vec!["host 1".to_string(), "host 2".to_string()];
    let tasks = vec![
        Task::Upload {
            remote: Path::new("/tmp/up").to_path_buf(),
            local: Path::new("packages/up").to_path_buf(),
            group: None,
            mode: None,
            owner: None,
        },
        Task::Download {
            remote: Path::new("/tmp/doanload").to_path_buf(),
            local: Path::new("down").to_path_buf(),
            group: None,
            mode: None,
            owner: None,
        },
        Task::Shell {
            script: "uname -a".to_string(),
            user: None,
        },
    ];
    let jobs = vec![
        Job {
            name: "job 1".to_string(),
            groups: groups.clone(),
            hosts: hosts.clone(),
            tasks: tasks.clone(),
            vars: vars.clone(),
        },
        Job {
            name: "job 2".to_string(),
            groups: groups.clone(),
            hosts: hosts.clone(),
            tasks: tasks.clone(),
            vars: vars.clone(),
        },
        Job {
            name: "job 3".to_string(),
            groups: groups.clone(),
            hosts: hosts.clone(),
            tasks: tasks.clone(),
            vars: vars.clone(),
        },
    ];
    println!(
        "{}",
        toml::to_string_pretty(&Role {
            name: "role 1".to_string(),
            jobs: jobs,
            vars
        })
        .unwrap()
    )
}
