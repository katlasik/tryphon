# Tryphon

A type-safe Rust library for loading configuration from environment variables using derive macros.

[![Crates.io](https://img.shields.io/crates/v/tryphon.svg)](https://crates.io/crates/tryphon)
[![Documentation](https://docs.rs/tryphon/badge.svg)](https://docs.rs/tryphon)

## Installation

```toml
[dependencies]
tryphon = "0.2.0"
```

## Quick Example

```rust
use tryphon::{Config, Secret};

#[derive(Debug, Config)]
struct AppConfig {
    #[env("DATABASE_URL")]
    database_url: String,

    #[env("API_KEY")]
    api_key: Secret<String>,

    #[env("PORT")]
    #[default(8080)]
    port: u16,
}

match AppConfig::load() {
    Ok(config) => {
        println!("Server starting on port {}", config.port);
    }
    Err(e) => {
        eprintln!("Configuration error: {}", e);
    }
}

#[test]
#[env_vars(API_KEY = "qwerty", DATABASE_URL = "http://localhost:5432")] //override env vars for test
fn test() {
  let config = AppConfig::load().expect("Failed to load test config");

  assert_eq!(*config.api_key, "qwerty");
  assert_eq!(config.database_url, "http://localhost:5432");
  assert_eq!(config.port, 8080);
}

```

## Documentation

**[📚 Full Documentation on docs.rs](https://docs.rs/tryphon)**

For detailed usage examples, supported types, and API reference, see the full documentation.

## License

MIT License
