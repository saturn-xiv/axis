use axis::models::{Group, Vars};

#[test]
fn generate() {
    let mut vars = Vars::new();
    vars.insert("key 1".to_string(), "val 1".to_string());
    vars.insert("key 2".to_string(), "val 2".to_string());
    vars.insert("key 3".to_string(), "val 3".to_string());

    println!(
        "{}",
        toml::to_string(&Group {
            hosts: vec!["host 1".to_string(), "host 2".to_string()],
            vars: vars,
        })
        .unwrap()
    )
}
