//! # Tryphon
//!
//! A type-safe Rust library for loading configuration from environment variables using derive macros.
//!
//! ## Quick Start
//!
//! ```rust
//! use tryphon::{Config, Secret};
//!
//! #[derive(Debug, Config)]
//! struct AppConfig {
//!     #[env("DATABASE_URL")]
//!     database_url: String,
//!
//!     #[env("API_KEY")]
//!     api_key: Secret<String>,
//!
//!     #[env("PORT")]
//!     #[default(8080)]
//!     port: u16,
//! }
//!
//! # unsafe { std::env::set_var("DATABASE_URL", "postgres://localhost/mydb"); }
//! # unsafe { std::env::set_var("API_KEY", "secret123"); }
//! match AppConfig::load() {
//!     Ok(config) => {
//!         println!("Server starting on port {}", config.port);
//!         // api_key is masked in debug output
//!         println!("Config: {:?}", config);
//!     }
//!     Err(e) => {
//!         eprintln!("Configuration error: {:#?}", e);
//!     }
//! }
//! ```
//!
//! ## Attributes
//!
//! ### `#[env("VAR_NAME")]`
//!
//! Specifies which environment variable to read for this field. You can provide multiple
//! `#[env]` attributes to create a fallback chain - they will be tried in order.
//!
//! ```rust
//! # use tryphon::Config;
//! #[derive(Config)]
//! struct AppConfig {
//!     #[env("APP_PORT")]
//!     #[env("PORT")]  // Fallback if APP_PORT is not set
//!     port: u16,
//! }
//! ```
//!
//! ### `#[default(value)]`
//!
//! Provides a default value to use if no environment variable is set.
//!
//! ```rust
//! # use tryphon::Config;
//! #[derive(Config)]
//! struct ServerConfig {
//!     #[env("HOST")]
//!     #[default("localhost")]
//!     host: String,
//!
//!     #[env("PORT")]
//!     #[default(8080)]
//!     port: u16,
//! }
//! ```
//!
//! ### `#[config]`
//!
//! Marks a field as a nested configuration that should be loaded recursively.
//! The field type must also implement `Config`.
//!
//! ```rust
//! # use tryphon::Config;
//! #[derive(Config)]
//! struct DatabaseConfig {
//!     #[env("DB_HOST")]
//!     host: String,
//! }
//!
//! #[derive(Config)]
//! struct AppConfig {
//!     #[config]  // Load nested config
//!     database: DatabaseConfig,
//! }
//! ```
//!
//! ## Usage Examples
//!
//! ### Basic Configuration
//!
//! Use the `#[env]` attribute to specify which environment variable to read:
//!
//! ```rust
//! use tryphon::Config;
//!
//! #[derive(Config)]
//! struct AppConfig {
//!     #[env("APP_NAME")]
//!     name: String,
//!
//!     #[env("MAX_CONNECTIONS")]
//!     max_connections: u32,
//! }
//! ```
//!
//! ### Optional Values
//!
//! Use `Option<T>` for values that may not be set:
//!
//! ```rust
//! # use tryphon::Config;
//! #[derive(Config)]
//! struct AppConfig {
//!     #[env("LOG_LEVEL")]
//!     log_level: Option<String>,  // None if environment variable not set
//!
//!     #[env("DEBUG_MODE")]
//!     debug: Option<bool>,
//! }
//! ```
//!
//! ### Secret Masking
//!
//! Use [`Secret<T>`] to prevent sensitive values from appearing in logs:
//!
//! ```rust
//! # use tryphon::{Config, Secret};
//! #[derive(Config)]
//! struct AppConfig {
//!     #[env("DB_PASSWORD")]
//!     password: Secret<String>,
//!
//!     #[env("API_TOKEN")]
//!     api_token: Secret<String>,
//! }
//!
//! # unsafe { std::env::set_var("DB_PASSWORD", "secret"); }
//! # unsafe { std::env::set_var("API_TOKEN", "token"); }
//! let config = AppConfig::load().unwrap();
//! # // Secrets show as "***" in output
//! # let password: &String = &config.password;
//! # assert_eq!(password, "secret");
//! ```
//!
//! ### Enum Configurations
//!
//! Use enums to handle different deployment scenarios. The library will try each variant
//! until one loads successfully:
//!
//! ```rust
//! # use tryphon::Config;
//! #[derive(Config)]
//! enum DatabaseConfig {
//!     Postgres {
//!         #[env("POSTGRES_URL")]
//!         url: String,
//!     },
//!     Sqlite {
//!         #[env("SQLITE_PATH")]
//!         path: String,
//!     },
//! }
//! ```
//!
//! ## Custom Type Decoders
//!
//! ### Using the Derive Macro
//!
//! For simple enums, use the `#[derive(ConfigValueDecoder)]` macro:
//!
//! ```rust
//! use tryphon::{ConfigValueDecoder, Config};
//!
//! #[derive(Debug, ConfigValueDecoder)]
//! enum LogLevel {
//!     Error,
//!     Warn,
//!     Info,
//!     Debug,
//! }
//!
//! #[derive(Config)]
//! struct AppConfig {
//!     #[env("LOG_LEVEL")]
//!     log_level: LogLevel,
//! }
//! ```
//!
//! ### Manual Implementation
//!
//! For more complex types, implement the [`ConfigValueDecoder`] trait:
//!
//! ```rust
//! use tryphon::{ConfigValueDecoder, Config};
//!
//! #[derive(Debug)]
//! struct Percentage(f64);
//!
//! impl ConfigValueDecoder for Percentage {
//!     fn decode(raw: String) -> Result<Self, String> {
//!         let value: f64 = raw.parse()
//!             .map_err(|e| format!("Failed to parse percentage: {}", e))?;
//!
//!         if value < 0.0 || value > 100.0 {
//!             return Err("Percentage must be between 0 and 100".to_string());
//!         }
//!
//!         Ok(Percentage(value))
//!     }
//! }
//! ```
//!
//! ## Supported Types
//!
//! Tryphon includes built-in decoders for:
//!
//! - **Primitives**: `String`, `bool`, `char`
//! - **Integers**: `u8`, `u16`, `u32`, `u64`, `u128`, `usize`, `i8`, `i16`, `i32`, `i64`, `i128`, `isize`
//! - **Floats**: `f32`, `f64`
//! - **Non-zero integers**: `NonZeroU8`, `NonZeroU16`, `NonZeroU32`, `NonZeroU64`, `NonZeroU128`,
//!   `NonZeroUsize`, `NonZeroI8`, `NonZeroI16`, `NonZeroI32`, `NonZeroI64`, `NonZeroI128`, `NonZeroIsize`
//! - **Network types**: `IpAddr`, `Ipv4Addr`, `Ipv6Addr`, `SocketAddr`, `SocketAddrV4`, `SocketAddrV6`
//! - **Path types**: `PathBuf`
//! - **Wrappers**: `Option<T>`, `Secret<T>` (for any `T` that implements [`ConfigValueDecoder`])
//!
//! ## Error Handling
//!
//! Tryphon collects all configuration errors and returns them together, making it easy
//! to see all issues at once. The [`ConfigError`] type provides a [`pretty_print`] method
//! with two formatting modes:
//!
//! ### List Mode (Compact)
//!
//! ```rust
//! use tryphon::{Config, ErrorPrintMode};
//!
//! #[derive(Config)]
//! struct AppConfig {
//!     #[env("REQUIRED_VAR")]
//!     required: String,
//! }
//!
//! match AppConfig::load() {
//!     Ok(config) => { /* use config */ }
//!     Err(e) => {
//!         // Compact list format, suitable for logs
//!         eprintln!("{}", e.pretty_print(ErrorPrintMode::List));
//!         // Output:
//!         // Found 1 configuration error(s):
//!         // Missing value for field 'required', tried env vars: REQUIRED_VAR
//!     }
//! }
//! ```
//!
//! ### Table Mode (Structured)
//!
//! ```rust
//! use tryphon::{Config, ErrorPrintMode};
//!
//! #[derive(Config)]
//! struct AppConfig {
//!     #[env("PORT")]
//!     port: u16,
//! }
//!
//! # unsafe { std::env::set_var("PORT", "invalid"); }
//! match AppConfig::load() {
//!     Ok(config) => { /* use config */ }
//!     Err(e) => {
//!         // ASCII table format, ideal for terminal output
//!         eprintln!("{}", e.pretty_print(ErrorPrintMode::Table));
//!         // Output:
//!         // ┌────────────┬────────────────────────┬─────────────────────────┐
//!         // │ Field Name │ Environment Variables  │ Error Details           │
//!         // ├────────────┼────────────────────────┼─────────────────────────┤
//!         // │ port       │ PORT                   │ invalid digit found...  │
//!         // └────────────┴────────────────────────┴─────────────────────────┘
//!     }
//! }
//! # unsafe { std::env::remove_var("PORT"); }
//! ```
//!
//! You can also access individual errors programmatically:
//!
//! ```rust
//! # use tryphon::Config;
//! # #[derive(Config)]
//! # struct AppConfig {
//! #     #[env("REQUIRED_VAR")]
//! #     required: String,
//! # }
//! match AppConfig::load() {
//!     Ok(config) => { /* use config */ }
//!     Err(e) => {
//!         for error in &e.field_errors {
//!             eprintln!("  - {:?}", error);
//!         }
//!     }
//! }
//! ```
//!
//! Error types include:
//! - [`ConfigFieldError::MissingValue`] - Required environment variable not set
//! - [`ConfigFieldError::ParsingError`] - Failed to parse value into target type
//! - [`ConfigFieldError::Nested`] - Error in nested configuration
//! - [`ConfigFieldError::Other`] - Custom error messages
//!
//! [`ConfigError`]: crate::ConfigError
//! [`pretty_print`]: crate::ConfigError::pretty_print

#[doc = include_str!("../../README.md")]
pub mod config;
pub mod config_error;
pub mod config_field_error;
pub mod config_value_decoder;
pub mod decoders;
pub mod error_print_mode;
mod printer;
pub mod secret;

pub use config::*;
pub use config_error::*;
pub use config_field_error::*;
pub use config_value_decoder::*;
pub use error_print_mode::*;
pub use secret::*;
pub use tryphon_macros::*;
