use crate::config_error::ConfigError;

/// A trait for types that can be loaded from environment variables.
///
/// This trait is typically implemented automatically using the `#[derive(Config)]` macro.
/// The generated implementation reads environment variables based on `#[env]` attributes,
/// applies default values from `#[default]` attributes, and handles nested configurations
/// marked with `#[config]`.
///
/// # Examples
///
/// ```rust
/// use tryphon::Config;
///
/// #[derive(Config)]
/// struct AppConfig {
///     #[env("DATABASE_URL")]
///     database_url: String,
///
///     #[env("PORT")]
///     #[default(8080)]
///     port: u16,
/// }
///
/// # unsafe { std::env::set_var("DATABASE_URL", "postgres://localhost/mydb"); }
/// let config = AppConfig::load().expect("Failed to load config");
/// assert_eq!(config.port, 8080);
/// ```
///
/// # Error Handling
///
/// The `load()` method returns a [`ConfigError`] which contains all field errors
/// encountered during loading. This allows you to see all configuration problems
/// at once rather than failing on the first error.
///
/// ```rust
/// use tryphon::Config;
///
/// #[derive(Config)]
/// struct AppConfig {
///     #[env("REQUIRED_VAR")]
///     required: String,
///
///     #[env("PORT")]
///     port: u16,
/// }
///
/// match AppConfig::load() {
///     Ok(config) => { /* use config */ }
///     Err(e) => {
///         eprintln!("Configuration errors:");
///         for error in &e.field_errors {
///             eprintln!("  - {:?}", error);
///         }
///     }
/// }
/// ```
pub trait Config {
    /// Loads the configuration from environment variables.
    ///
    /// Reads all required environment variables, applies defaults, validates fields,
    /// and returns either a fully constructed configuration or a collection of all
    /// errors encountered.
    ///
    /// # Errors
    ///
    /// Returns [`ConfigError`] if any field fails to load. The error contains a
    /// vector of all individual field errors, allowing you to diagnose all
    /// configuration problems at once.
    fn load() -> Result<Self, ConfigError>
    where
        Self: Sized;
}
