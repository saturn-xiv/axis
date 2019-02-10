extern crate axis;

use axis::agent::task::Payload;

#[test]
fn it_shell() {
    for it in vec![
        Payload::Shell {
            user: "root".to_string(),
            script: "uname -a".to_string(),
        },
        Payload::Shell {
            user: "root".to_string(),
            script: r#"echo "$PATH""#.to_string(),
        },
        Payload::Shell {
            user: "root".to_string(),
            script: "whoami".to_string(),
        },
        Payload::Shell {
            user: "jeremy".to_string(),
            script: "date".to_string(),
        },
        Payload::Shell {
            user: "jeremy".to_string(),
            script: r#"echo "$PATH""#.to_string(),
        },
        Payload::Shell {
            user: "jeremy".to_string(),
            script: "whoami".to_string(),
        },
    ] {
        println!("run {}", it);
        println!("result {}", it.execute().unwrap());
    }
}
