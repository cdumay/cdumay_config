# cdumay_config

[![License: BSD-3-Clause](https://img.shields.io/badge/license-BSD--3--Clause-blue)](./LICENSE)
[![cdumay_config on crates.io](https://img.shields.io/crates/v/cdumay_config)](https://crates.io/crates/cdumay_config)
[![cdumay_config on docs.rs](https://docs.rs/cdumay_config/badge.svg)](https://docs.rs/cdumay_config)
[![Source Code Repository](https://img.shields.io/badge/Code-On%20GitHub-blue?logo=GitHub)](https://github.com/cdumay/cdumay_config)

A flexible configuration management library that provides a trait-based approach for handling
key-value data with support of multiple serialization formats.

## Features

- Generic configuration management through the `Manager` trait
- Support for multiple serialization formats (with feature flags):
  - JSON (default)
  - TOML (feature: "toml")
  - YAML (feature: "yaml")
  - XML (feature: "xml")
- Type-safe error handling with the `cdumay_error::Error` struct

## Example Usage

```rust
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct DatabaseConfig {
    pub user: String,
    pub password: String,
    pub database: String,
}

fn main() -> cdumay_error::Result<()> {
    let context = std::collections::BTreeMap::new();
    let config = DatabaseConfig {
        user: "john".to_string(),
        password: "smith".to_string(),
        database: "example".to_string()
    };
    let _ = cdumay_config::write_config(
        "locker-db.json",
        Some(cdumay_config::ContentFormat::JSON),
        config,
        &context
    )?;
    Ok(())
}
```

