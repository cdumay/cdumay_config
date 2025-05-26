use std::collections::BTreeMap;
use std::io::{Cursor, Seek, SeekFrom, Write};

use cdumay_config::{Manager, YamlManager};
use serde::{Deserialize, Serialize};
use serde_value::Value;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct TestYamlConfig {
    project: String,
    version: u32,
}

fn default_context() -> BTreeMap<String, Value> {
    BTreeMap::new()
}

#[test]
fn test_yaml_manager_new_and_path() {
    let manager = YamlManager::new("test.yaml".to_string());
    assert_eq!(manager.path(), "test.yaml");
}

#[test]
fn test_yaml_manager_read_str_success() {
    let yaml = r#"
project: myapp
version: 1
"#;
    let context = default_context();
    let config: TestYamlConfig = YamlManager::read_str(yaml, &context).unwrap();
    assert_eq!(config.project, "myapp");
    assert_eq!(config.version, 1);
}

#[test]
fn test_yaml_manager_read_str_failure() {
    let yaml = r#"
project: myapp
version: [not a number]
"#;
    let context = default_context();
    let result: cdumay_core::Result<TestYamlConfig> = YamlManager::read_str(yaml, &context);
    assert!(result.is_err());
}

#[test]
fn test_yaml_manager_read_success() {
    let yaml = b"project: read_app\nversion: 3\n";
    let reader = Cursor::new(yaml);
    let context = default_context();
    let manager = YamlManager::new("read.yaml".to_string());
    let config: TestYamlConfig = manager.read(reader, &context).unwrap();
    assert_eq!(config.project, "read_app");
    assert_eq!(config.version, 3);
}

#[test]
fn test_yaml_manager_read_failure() {
    let yaml = b"project: read_app\nversion: [oops]\n";
    let reader = Cursor::new(yaml);
    let context = default_context();
    let manager = YamlManager::new("fail_read.yaml".to_string());
    let result: cdumay_core::Result<TestYamlConfig> = manager.read(reader, &context);
    assert!(result.is_err());
}

#[test]
fn test_yaml_manager_write_success() {
    let config = TestYamlConfig {
        project: "yaml_writer".to_string(),
        version: 7,
    };
    let context = default_context();
    let manager = YamlManager::new("write.yaml".to_string());
    let mut buffer = Cursor::new(Vec::new());

    manager.write(&mut buffer, &config, &context).unwrap();
    buffer.seek(SeekFrom::Start(0)).unwrap();

    let deserialized: TestYamlConfig = serde_yaml::from_reader(buffer).unwrap();
    assert_eq!(deserialized, config);
}

#[test]
fn test_yaml_manager_write_failure_on_writer() {
    struct FailingWriter;

    impl Write for FailingWriter {
        fn write(&mut self, _buf: &[u8]) -> std::io::Result<usize> {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "Simulated write failure"))
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    let context = default_context();
    let config = TestYamlConfig {
        project: "fail_writer".to_string(),
        version: 99,
    };
    let manager = YamlManager::new("fail.yaml".to_string());

    let result = manager.write(FailingWriter, &config, &context);
    assert!(result.is_err());
}
