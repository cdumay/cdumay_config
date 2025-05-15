use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::io::{Cursor, Write};

use cdumay_config::{Manager, TomlManager};
use serde_value::Value;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct TestConfig {
    name: String,
    value: i32,
}

fn default_context() -> BTreeMap<String, Value> {
    BTreeMap::new()
}

#[test]
fn test_toml_manager_new_and_path() {
    let manager = TomlManager::new("example.toml".to_string());
    assert_eq!(manager.path(), "example.toml");
}

#[test]
fn test_toml_manager_read_str_success() {
    let toml = r#"name = "alpha"
value = 42"#;
    let context = default_context();
    let result: TestConfig = TomlManager::read_str(toml, &context).unwrap();
    assert_eq!(result.name, "alpha");
    assert_eq!(result.value, 42);
}

#[test]
fn test_toml_manager_read_str_failure() {
    let toml = r#"name = "broken
value = 42"#;
    let context = default_context();
    let result: cdumay_error::Result<TestConfig> = TomlManager::read_str(toml, &context);
    assert!(result.is_err());
}

#[test]
fn test_toml_manager_read_success() {
    let toml = r#"name = "reader"
value = 100"#;
    let reader = Cursor::new(toml);
    let context = default_context();
    let manager = TomlManager::new("reader.toml".to_string());
    let result: TestConfig = manager.read(reader, &context).unwrap();
    assert_eq!(result.name, "reader");
    assert_eq!(result.value, 100);
}

#[test]
fn test_toml_manager_read_invalid_toml() {
    let toml = r#"name = "bad
value = 100"#;
    let reader = Cursor::new(toml);
    let context = default_context();
    let manager = TomlManager::new("broken.toml".to_string());
    let result: cdumay_error::Result<TestConfig> = manager.read(reader, &context);
    assert!(result.is_err());
}

#[test]
fn test_toml_manager_write_success() {
    let config = TestConfig {
        name: "write".to_string(),
        value: 99,
    };

    let context = default_context();
    let manager = TomlManager::new("write.toml".to_string());
    let mut buffer = Cursor::new(Vec::new());

    let result = manager.write(&mut buffer, &config, &context);
    assert!(result.is_ok());
}

#[test]
fn test_toml_manager_write_failure_on_write() {
    struct FailingWriter;

    impl Write for FailingWriter {
        fn write(&mut self, _buf: &[u8]) -> std::io::Result<usize> {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "write error"))
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    let config = TestConfig {
        name: "fail".to_string(),
        value: 0,
    };
    let context = default_context();
    let manager = TomlManager::new("fail.toml".to_string());

    let result = manager.write(FailingWriter, &config, &context);
    assert!(result.is_err());
}
