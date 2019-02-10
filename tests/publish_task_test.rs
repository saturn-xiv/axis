extern crate axis;
extern crate serde_json;

use axis::publish::models::Task;

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
            owner: Some("nobody".to_string()),
            group: Some("nobody".to_string()),
            mode: Some(0o400),
        },
    ];
    let buf = serde_json::to_string_pretty(&items).unwrap();
    println!("{}", buf);
    println!("{:?}", serde_json::from_str::<Vec<Task>>(&buf).unwrap());
}
