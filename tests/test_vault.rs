use cdumay_config::{ContentFormat, VaultConfig, VaultSecret, VaultSecrets};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct DummySecret {
    username: String,
    password: String,
}

fn sample_context() -> BTreeMap<String, serde_value::Value> {
    let mut ctx = BTreeMap::new();
    ctx.insert("env".to_string(), serde_value::Value::String("dev".to_string()));
    ctx
}

#[test]
fn test_secret_alias_json_success() {
    let context = sample_context();
    let json_value = r#"{"username": "admin", "password": "1234"}"#.to_string();
    let secrets = VaultSecrets::new(vec![VaultSecret::new("db", "db_key", &json_value)]);

    let result: DummySecret = secrets
        .alias("db".to_string(), ContentFormat::JSON, &context)
        .expect("Should deserialize");

    assert_eq!(
        result,
        DummySecret {
            username: "admin".to_string(),
            password: "1234".to_string()
        }
    );
}

#[test]
fn test_secret_alias_not_found() {
    let context = sample_context();
    let secrets = VaultSecrets::new(vec![]);

    let result: cdumay_core::Result<DummySecret> = secrets.alias("missing".to_string(), ContentFormat::JSON, &context);

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(format!("{}", err).contains("Invalid alias"));
}

#[cfg(feature = "yaml")]
#[test]
fn test_secret_alias_yaml_success() {
    let context = sample_context();
    let yaml_value = r#"username: admin
password: 1234"#
        .to_string();
    let secrets = VaultSecrets::new(vec![VaultSecret::new("db", "db_key", &yaml_value)]);

    let result: DummySecret = secrets
        .alias("db".to_string(), ContentFormat::YAML, &context)
        .expect("Should deserialize YAML");

    assert_eq!(
        result,
        DummySecret {
            username: "admin".to_string(),
            password: "1234".to_string()
        }
    );
}

#[test]
fn test_vault_config_init_and_secrets_success() {
    // Prepare JSON file with secrets
    use std::fs::File;
    use std::io::Write;

    let temp_file = tempfile::NamedTempFile::new().expect("temp file");
    let mut file = File::create(temp_file.path()).unwrap();

    let json_data = r#"[{
            "alias": "db",
            "key": "db_key",
            "value": "{\"username\": \"admin\", \"password\": \"1234\"}"
        }]"#;
    file.write_all(json_data.as_bytes()).unwrap();

    let context = sample_context();
    let config = VaultConfig::init(temp_file.path().to_str().unwrap(), &context).expect("Init failed");

    let secrets = config.secrets(&context).expect("Should return secrets");

    let result: DummySecret = secrets
        .alias("db".to_string(), ContentFormat::JSON, &context)
        .expect("Should deserialize");

    assert_eq!(result.username, "admin");
    assert_eq!(result.password, "1234");
}

#[test]
fn test_vault_config_secrets_none_error() {
    let config = VaultConfig { secrets: None };
    let context = sample_context();

    let result = config.secrets(&context);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(format!("{}", err).contains("Failed to read vault data"));
}
