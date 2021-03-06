use axis::models::Vars;
use toml::Value;

#[test]
fn generate() {
    let mut vars = Vars::new();
    vars.insert("key 1".to_string(), Value::String("val 1".to_string()));
    vars.insert("key 2".to_string(), Value::String("val 2".to_string()));
    vars.insert("key 3".to_string(), Value::String("val 3".to_string()));

    println!("{}", toml::to_string_pretty(&vars).unwrap())
}
