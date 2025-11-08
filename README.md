# Tryphon

A type-safe Rust library for loading configuration from environment variables using derive macros.

[![Crates.io](https://img.shields.io/crates/v/tryphon.svg)](https://crates.io/crates/tryphon)
[![Documentation](https://docs.rs/tryphon/badge.svg)](https://docs.rs/tryphon)

## Installation

```toml
[dependencies]
tryphon = "0.1.0"
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

```

## Documentation

**[📚 Full Documentation on docs.rs](https://docs.rs/tryphon)**

For detailed usage examples, supported types, and API reference, see the full documentation.

## License

MIT License
