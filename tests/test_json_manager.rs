use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::io::{Cursor, Seek, SeekFrom};

use cdumay_config::{JsonManager, Manager};
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
fn test_json_manager_new_and_path() {
    let manager = JsonManager::new("test.json".to_string());
    assert_eq!(manager.path(), "test.json");
}

#[test]
fn test_json_manager_read_str_success() {
    let json = r#"{ "name": "example", "value": 42 }"#;
    let context = default_context();
    let result: TestConfig = JsonManager::read_str(json, &context).unwrap();
    assert_eq!(result.name, "example");
    assert_eq!(result.value, 42);
}

#[test]
fn test_json_manager_read_str_failure() {
    let json = r#"{ "name": "example", "value": "not_an_int" }"#;
    let context = default_context();
    let result: Result<TestConfig, cdumay_core::Error> = JsonManager::read_str(json, &context);
    assert!(result.is_err());
}

#[test]
fn test_json_manager_read_success() {
    let json = r#"{ "name": "reader_test", "value": 10 }"#;
    let reader = Cursor::new(json);
    let context = default_context();
    let manager = JsonManager::new("dummy.json".to_string());

    let result: TestConfig = manager.read(reader, &context).unwrap();
    assert_eq!(result.name, "reader_test");
    assert_eq!(result.value, 10);
}

#[test]
fn test_json_manager_read_failure() {
    let json = r#"{ "name": "bad", "value": "oops" }"#;
    let reader = Cursor::new(json);
    let context = default_context();
    let manager = JsonManager::new("dummy.json".to_string());

    let result: Result<TestConfig, cdumay_core::Error> = manager.read(reader, &context);
    assert!(result.is_err());
}

#[test]
fn test_json_manager_write_success() {
    let data = TestConfig {
        name: "write_test".to_string(),
        value: 123,
    };

    let context = default_context();
    let manager = JsonManager::new("write.json".to_string());
    let mut buffer = Cursor::new(Vec::new());

    manager.write(&mut buffer, &data, &context).unwrap();

    buffer.seek(SeekFrom::Start(0)).unwrap();
    let written: TestConfig = serde_json::from_reader(buffer).unwrap();
    assert_eq!(written, data);
}

#[test]
fn test_json_manager_write_failure() {
    use std::io::{self, Write};

    struct BrokenWriter;

    impl Write for BrokenWriter {
        fn write(&mut self, _buf: &[u8]) -> io::Result<usize> {
            Err(io::Error::new(io::ErrorKind::Other, "write failed"))
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let context = default_context();
    let manager = JsonManager::new("broken.json".to_string());

    let data = TestConfig {
        name: "fail".to_string(),
        value: 0,
    };

    let result = manager.write(BrokenWriter, &data, &context);
    assert!(result.is_err());
}
