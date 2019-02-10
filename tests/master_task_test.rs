extern crate axis;
extern crate serde_json;

use axis::master::config::Task;

#[test]
fn it_generate() {
    let items = vec![
        Task::Script {
            user: "root".to_string(),
            file: "upgrade.sh".to_string(),
        },
        Task::Upload {
            source: "etc/aaa/conf".to_string(),
            target: "/opt/ect/aaa.conf".to_string(),
            owner: None,
            group: None,
            mode: None,
        },
        Task::Upload {
            source: "etc/bbb".to_string(),
            target: "/opt/ect/bbb".to_string(),
            owner: None,
            group: None,
            mode: None,
        },
    ];
    let buf = serde_json::to_string_pretty(&items).unwrap();
    println!("{}", buf);
    println!("{:?}", serde_json::from_str::<Vec<Task>>(&buf).unwrap());
}

// #[test]
// fn it_uname() {
//     for it in vec![
//         Task::Shell{user:"root".to_string(), file:"aaa/aaa.sh".to_string()},
//         // Task::File((
//         //     "bbb/bbb.conf".to_string(),
//         //     Some("root".to_string()),
//         //     Some("root".to_string()),
//         //     0o755,
//         // )),
//     ] {
//         println!("run {:?}", it);
//         println!("result {:?}", toml::ser::to_string_pretty(&it));
//     }
// }
