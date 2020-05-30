use axis::models::{Command, Job, Task, Vars};
use toml::Value;

#[test]
fn generate() {
    let mut vars = Vars::new();
    vars.insert(
        "global key 1".to_string(),
        Value::String("global val 1".to_string()),
    );
    vars.insert(
        "global key 2".to_string(),
        Value::String("global val 2".to_string()),
    );
    vars.insert(
        "global key 3".to_string(),
        Value::String("global val 3".to_string()),
    );

    let mut tvr = Vars::new();
    tvr.insert(
        "task key 1".to_string(),
        Value::String("task val 1".to_string()),
    );
    tvr.insert(
        "task key 2".to_string(),
        Value::String("task val 2".to_string()),
    );
    tvr.insert(
        "task key 3".to_string(),
        Value::String("task val 3".to_string()),
    );

    let groups = vec![
        "group 1".to_string(),
        "group 2".to_string(),
        "group 3".to_string(),
    ];
    let commands = vec![
        Command::Upload {
            src: "tmp/uuu".to_string(),
            dest: "/etc/uuu".to_string(),
        },
        Command::Download {
            src: "/etc/ddd".to_string(),
            dest: "tmp/ddd".to_string(),
        },
        Command::Shell {
            script: "aaa.sh".to_string(),
        },
    ];
    println!(
        "{}",
        toml::to_string_pretty(&Job {
            tasks: vec![
                Task {
                    name: "task 1".to_string(),
                    groups: groups.clone(),
                    commands: commands.clone(),
                    vars: tvr.clone()
                },
                Task {
                    name: "task 2".to_string(),
                    groups: groups.clone(),
                    commands: commands.clone(),
                    vars: tvr.clone()
                },
                Task {
                    name: "task 3".to_string(),
                    groups: groups.clone(),
                    commands: commands.clone(),
                    vars: tvr.clone()
                }
            ],
            vars
        })
        .unwrap()
    )
}
