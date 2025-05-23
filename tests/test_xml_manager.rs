use std::collections::BTreeMap;
use std::io::{Cursor, Seek, SeekFrom, Write};

use cdumay_config::{Manager, XmlManager};
use serde::{Deserialize, Serialize};
use serde_value::Value;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct TestXmlConfig {
    name: String,
    count: i32,
}

fn default_context() -> BTreeMap<String, Value> {
    BTreeMap::new()
}

#[test]
fn test_xml_manager_new_and_path() {
    let manager = XmlManager::new("test.xml".to_string());
    assert_eq!(manager.path(), "test.xml");
}

#[test]
fn test_xml_manager_read_str_success() {
    let xml = r#"<TestXmlConfig><name>config</name><count>5</count></TestXmlConfig>"#;
    let context = default_context();
    let result: TestXmlConfig = XmlManager::read_str(xml, &context).unwrap();
    assert_eq!(result.name, "config");
    assert_eq!(result.count, 5);
}

#[test]
fn test_xml_manager_read_str_failure() {
    let xml = r#"<TestXmlConfig><name>bad<name><count>5</count></TestXmlConfig>"#;
    let context = default_context();
    let result: Result<TestXmlConfig, cdumay_core::Error> = XmlManager::read_str(xml, &context);
    assert!(result.is_err());
}

#[test]
fn test_xml_manager_read_success() {
    let xml = r#"<TestXmlConfig><name>read</name><count>42</count></TestXmlConfig>"#;
    let reader = Cursor::new(xml);
    let context = default_context();
    let manager = XmlManager::new("read.xml".to_string());
    let result: TestXmlConfig = manager.read(reader, &context).unwrap();
    assert_eq!(result.name, "read");
    assert_eq!(result.count, 42);
}

#[test]
fn test_xml_manager_read_invalid() {
    let xml = r#"<TestXmlConfig><name>bad<name><count>5</count></TestXmlConfig>"#;
    let reader = Cursor::new(xml);
    let context = default_context();
    let manager = XmlManager::new("fail_read.xml".to_string());
    let result: Result<TestXmlConfig, cdumay_core::Error> = manager.read(reader, &context);
    assert!(result.is_err());
}

#[test]
fn test_xml_manager_write_success() {
    let config = TestXmlConfig {
        name: "writer".to_string(),
        count: 7,
    };
    let context = default_context();
    let manager = XmlManager::new("write.xml".to_string());
    let mut buffer = Cursor::new(Vec::new());

    manager.write(&mut buffer, &config, &context).unwrap();

    buffer.seek(SeekFrom::Start(0)).unwrap();
    let deserialized: TestXmlConfig = serde_xml_rs::from_reader(buffer).unwrap();
    assert_eq!(deserialized, config);
}

#[test]
fn test_xml_manager_write_failure_on_write() {
    struct FailingWriter;

    impl Write for FailingWriter {
        fn write(&mut self, _buf: &[u8]) -> std::io::Result<usize> {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "Simulated write error"))
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    let context = default_context();
    let manager = XmlManager::new("failing.xml".to_string());
    let config = TestXmlConfig {
        name: "fail".to_string(),
        count: 0,
    };

    let result = manager.write(FailingWriter, &config, &context);
    assert!(result.is_err());
}
