use axis::models::{Command, Job, Task, Vars};

#[test]
fn generate() {
    let mut vars = Vars::new();
    vars.insert("global key 1".to_string(), "global val 1".to_string());
    vars.insert("global key 2".to_string(), "global val 2".to_string());
    vars.insert("global key 3".to_string(), "global val 3".to_string());

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
                    commands: commands.clone()
                },
                Task {
                    name: "task 2".to_string(),
                    groups: groups.clone(),
                    commands: commands.clone()
                },
                Task {
                    name: "task 3".to_string(),
                    groups: groups.clone(),
                    commands: commands.clone()
                }
            ],
            vars
        })
        .unwrap()
    )
}
