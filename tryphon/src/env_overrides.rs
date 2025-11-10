//! Environment variable overrides for testing.
//!
//! This module provides a thread-local mechanism to override environment variables during tests,
//! allowing tests to run in parallel without interfering with each other or modifying the global
//! environment state.
//!
//! # Overview
//!
//! When testing configuration loading, you typically need to set environment variables. However,
//! environment variables are global to the process, which causes problems:
//!
//! - Tests that modify environment variables cannot run in parallel
//! - Environment changes persist between tests unless manually cleaned up
//! - Tests can interfere with each other
//!
//! The `EnvOverrides` type solves this by providing thread-local environment variable overrides.
//! When a test initializes `EnvOverrides`, all config loading in that thread will use the override
//! values instead of the actual environment variables.
//!
//! # Usage
//!
//! ## Basic Usage
//!
//! ```rust
//! use tryphon::Config;
//!
//! #[derive(Config)]
//! struct TestConfig {
//!     #[env("DATABASE_URL")]
//!     database_url: String,
//! }
//!
//! #[test]
//! fn test_config_loading() {
//!     let mut overrides = EnvOverrides::init()
//!         .set("DATABASE_URL", "postgres://test-db");
//!
//!     let config = TestConfig::load().unwrap();
//!     assert_eq!(config.database_url, "postgres://test-db");
//!
//! }
//! ```
//!
//! ## Using the `#[env_vars]` Attribute Macro
//!
//! You can use the `#[env_vars]` attribute macro to automatically set up environment variable overrides for your test:
//!
//! ```rust
//! use tryphon::{Config, env_vars};
//!
//! #[derive(Config)]
//! struct TestConfig {
//!     #[env("FOO")]
//!     foo: String,
//!     #[env("BAZ")]
//!     baz: String,
//! }
//!
//! #[test]
//! #[env_vars(FOO = "bar", BAZ = "qux")]
//! fn test_with_attribute() {
//!     let config = TestConfig::load().unwrap();
//!     assert_eq!(config.foo, "bar");
//!     assert_eq!(config.baz, "qux");
//! }
//! ```
//!
//! ## Parallel Test Execution
//!
//! The main advantage is that tests can run in parallel without conflicts:
//!
//! ```rust
//! use tryphon::{Config, EnvOverrides};
//!
//! #[derive(Config)]
//! struct TestConfig {
//!     #[env("VALUE")]
//!     value: String,
//! }
//!
//! #[test]
//! fn test_parallel_1() {
//!     let mut overrides = EnvOverrides::init();
//!     overrides.set("VALUE", "test1");
//!     let config = TestConfig::load().unwrap();
//!     assert_eq!(config.value, "test1");
//! }
//!
//! #[test]
//! fn test_parallel_2() {
//!     let mut overrides = EnvOverrides::init();
//!     overrides.set("VALUE", "test2");
//!     let config = TestConfig::load().unwrap();
//!     assert_eq!(config.value, "test2");
//! }
//! // These tests can run simultaneously without interference
//! ```
//!
//! # Implementation Details
//!
//! The overrides are stored in a thread-local `HashMap`. When [`crate::read_env()`] is called,
//! it first checks if overrides are initialized in the current thread. If so, it returns the
//! override value (or `NotPresent` error if not set). Otherwise, it falls back to reading the
//! actual environment variable.
//!
//! The `EnvOverrides` struct uses RAII (Resource Acquisition Is Initialization) to ensure cleanup:
//! when the instance is dropped, the overrides for that thread are cleared.

use std::cell::RefCell;
use std::collections::HashMap;

thread_local! {
  static TEST_OVERRIDES: RefCell<Option<HashMap<String, String>,> >= RefCell::new(None);
}

/// Thread-local environment variable overrides for testing.
///
/// This type allows you to override environment variables for config loading within a single thread,
/// enabling parallel test execution without global environment modification.
///
/// # Example
///
/// ```rust
/// use tryphon::{Config, EnvOverrides};
///
/// #[derive(Config)]
/// struct AppConfig {
///     #[env("API_KEY")]
///     api_key: String,
/// }
///
/// let mut overrides = EnvOverrides::init();
/// overrides.set("API_KEY", "test-key-123");
///
/// let config = AppConfig::load().unwrap();
/// assert_eq!(config.api_key, "test-key-123");
/// ```
///
/// # Panics
///
/// [`init()`](EnvOverrides::init) panics if called when overrides are already initialized in the current thread.
/// You must not create multiple `EnvOverrides` instances in the same thread.
pub struct EnvOverrides();

impl EnvOverrides {
    /// Initializes environment variable overrides for the current thread.
    ///
    /// This must be called before using [`set()`](EnvOverrides::set) to configure override values.
    /// The returned `EnvOverrides` instance manages the lifecycle of the overrides - when it's
    /// dropped, the overrides are automatically cleaned up.
    ///
    /// # Panics
    ///
    /// Panics if overrides are already initialized in the current thread. You must not create
    /// multiple `EnvOverrides` instances in the same thread.
    ///
    /// # Example
    ///
    /// ```rust
    ///
    /// let mut overrides = tryphon::EnvOverrides::init();
    /// overrides.set("KEY", "value");
    /// ```
    pub fn init() -> EnvOverrides {
        TEST_OVERRIDES.with(|overrides| {
          let mut overrides = overrides.borrow_mut();

          if overrides.is_some() {
            panic!("TestOverrides already initialized. You must not create multiple instances of TestOverrides for single thread.");
          } else {
            *overrides = Some(HashMap::new());
          }
        });

        EnvOverrides()
    }

    /// Sets an environment variable override for the current thread.
    ///
    /// After calling this, any config loading in the current thread that would normally read
    /// the specified environment variable will receive the override value instead.
    ///
    ///
    /// # Example
    ///
    /// ```rust
    /// use tryphon::{Config, EnvOverrides};
    ///
    /// #[derive(Config)]
    /// struct TestConfig {
    ///     #[env("PORT")]
    ///     port: u16,
    ///     #[env("HOST")]
    ///     host: String,
    /// }
    ///
    /// let mut overrides = EnvOverrides::init();
    /// overrides.set("PORT", "8080")
    ///          .set("HOST", "localhost");
    ///
    /// let config = TestConfig::load().unwrap();
    /// assert_eq!(config.port, 8080);
    /// assert_eq!(config.host, "localhost");
    /// ```
    pub fn set(&mut self, key: &str, value: &str) -> &mut Self {
        TEST_OVERRIDES.with(|overrides| {
            let mut overrides = overrides.borrow_mut();
            if let Some(ref mut to) = *overrides {
                to.insert(key.to_string(), value.to_string());
            } else {
                panic!("TestOverrides not initialized.");
            }
        });
        self
    }

    /// Gets an override value for the specified environment variable key.
    ///
    /// Returns `Some(value)` if an override is set for this key in the current thread,
    /// or `None` if no override exists or overrides are not initialized.
    ///
    /// Note: This is used internally by [`crate::read_env()`] and typically doesn't
    /// need to be called directly in tests.
    ///
    /// # Example
    ///
    /// ```rust
    /// use tryphon::{Config, EnvOverrides};
    ///
    /// let mut overrides = EnvOverrides::init();
    /// overrides.set("KEY", "value");
    ///
    /// assert_eq!(EnvOverrides::get("KEY"), Some("value".to_string()));
    /// assert_eq!(EnvOverrides::get("OTHER"), None);
    /// ```
    pub fn get(key: &str) -> Option<String> {
        TEST_OVERRIDES.with(|overrides| {
            let overrides = overrides.borrow();
            if let Some(ref to) = *overrides {
                to.get(key).cloned()
            } else {
                None
            }
        })
    }

    /// Checks if environment variable overrides are initialized for the current thread.
    ///
    /// Returns `true` if [`init()`](EnvOverrides::init) has been called and the overrides
    /// haven't been dropped yet, `false` otherwise.
    ///
    /// Note: This is used internally by [`crate::read_env()`] and typically doesn't
    /// need to be called directly in tests.
    ///
    /// # Example
    ///
    /// ```rust
    /// use tryphon::EnvOverrides;
    ///
    /// assert!(!EnvOverrides::is_initialized());
    ///
    /// {
    ///     let _overrides = EnvOverrides::init();
    ///     assert!(EnvOverrides::is_initialized());
    /// }
    ///
    /// assert!(!EnvOverrides::is_initialized()); // Cleaned up after drop
    /// ```
    pub fn is_initialized() -> bool {
        TEST_OVERRIDES.with(|overrides| {
            let overrides = overrides.borrow();
            overrides.is_some()
        })
    }
}

impl Drop for EnvOverrides {
    fn drop(&mut self) {
        TEST_OVERRIDES.with(|overrides| {
            let mut overrides = overrides.borrow_mut();
            if overrides.is_some() {
                *overrides = None;
            } else {
                panic!("TestOverrides not initialized.");
            }
        });
    }
}
