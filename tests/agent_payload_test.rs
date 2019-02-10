extern crate axis;

use axis::agent::task::Payload;

#[test]
fn it_shell() {
    for it in vec![
        Payload::Shell(("root".to_string(), "uname -a".to_string())),
        Payload::Shell(("root".to_string(), r#"echo "$PATH""#.to_string())),
        Payload::Shell(("root".to_string(), "whoami".to_string())),
        Payload::Shell(("jeremy".to_string(), "date".to_string())),
        Payload::Shell(("jeremy".to_string(), r#"echo "$PATH""#.to_string())),
        Payload::Shell(("jeremy".to_string(), "whoami".to_string())),
    ] {
        println!("run {}", it);
        println!("result {}", it.execute().unwrap());
    }
}
