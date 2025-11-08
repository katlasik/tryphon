use crate::config_error::ConfigError;

/// Represents an error that occurred while loading a specific configuration field.
///
/// Each variant provides detailed information about what went wrong, including the
/// field name, environment variable names, and error messages to help diagnose
/// configuration issues.
///
/// # Variants
///
/// * [`ParsingError`](ConfigFieldError::ParsingError) - Failed to parse the environment variable value into the target type
/// * [`MissingValue`](ConfigFieldError::MissingValue) - Required environment variable(s) not set
/// * [`Nested`](ConfigFieldError::Nested) - Error in a nested configuration field
/// * [`Other`](ConfigFieldError::Other) - A custom error with a message
#[derive(Debug, Clone)]
pub enum ConfigFieldError {
    /// Failed to parse an environment variable value into the target type.
    ///
    /// This error occurs when an environment variable is set but its value cannot
    /// be converted to the expected type (e.g., "abc" for a `u16` field).
    ///
    /// # Fields
    ///
    /// * `field_name` - The name of the configuration field that failed
    /// * `raw` - The raw string value from the environment variable
    /// * `message` - A detailed error message explaining why parsing failed
    /// * `env_var_name` - The name of the environment variable that was read
    ///
    /// # Example
    ///
    /// ```rust
    /// use tryphon::Config;
    ///
    /// #[derive(Debug, Config)]
    /// struct ServerConfig {
    ///     #[env("PORT")]
    ///     port: u16,
    /// }
    ///
    /// # unsafe { std::env::set_var("PORT", "not-a-number"); }
    /// let err = ServerConfig::load().unwrap_err();
    /// // Will contain a ParsingError
    /// ```
    ParsingError {
        /// The index of the field in the struct.
        field_idx: usize,
        /// The name of the configuration field that failed to parse.
        field_name: Option<String>,
        /// The raw string value that failed to parse.
        raw: String,
        /// A detailed error message explaining the parsing failure.
        message: String,
        /// The name of the environment variable that was read.
        env_var_name: String,
    },

    /// Required environment variable(s) are not set.
    ///
    /// This error occurs when a field requires an environment variable but none
    /// of the specified variables are set (and no default value is provided).
    ///
    /// # Fields
    ///
    /// * `field_name` - The name of the configuration field that's missing
    /// * `field_idx` - The index of the field in the struct (for debugging)
    /// * `env_vars` - All environment variable names that were tried (in order)
    ///
    /// # Example
    ///
    /// ```rust
    /// use tryphon::Config;
    ///
    /// #[derive(Debug, Config)]
    /// struct DbConfig {
    ///     #[env("DATABASE_URL")]
    ///     #[env("DB_URL")]  // Fallback
    ///     database_url: String,
    /// }
    ///
    /// // Neither DATABASE_URL nor DB_URL are set
    /// let err = DbConfig::load().unwrap_err();
    /// // Will contain a MissingValue error with both variable names
    /// ```
    MissingValue {
        /// The name of the configuration field that's missing.
        field_name: Option<String>,
        /// The index of the field in the struct.
        field_idx: usize,
        /// All environment variable names that were tried (in fallback order).
        env_vars: Vec<String>,
    },

    /// A custom error with a specific message.
    ///
    /// Used for errors that don't fit into the other categories.
    ///
    /// # Fields
    ///
    /// * `field_name` - The name of the field that caused the error
    /// * `message` - A custom error message
    Other {
        /// The index of the field in the struct.
        field_idx: usize,
        /// The name of the field that caused the error.
        field_name: Option<String>,
        /// A custom error message.
        message: String,
    },

    /// Error occurred in a nested configuration field.
    ///
    /// When a field is marked with `#[config]`, it represents a nested configuration
    /// struct. If loading that nested struct fails, the errors are wrapped in this
    /// variant to maintain the error hierarchy.
    ///
    /// # Fields
    ///
    /// * `field_name` - The name of the nested configuration field
    /// * `error` - The [`ConfigError`] from the nested configuration
    ///
    /// # Example
    ///
    /// ```rust
    /// use tryphon::Config;
    ///
    /// #[derive(Debug, Config)]
    /// struct DatabaseConfig {
    ///     #[env("DB_HOST")]
    ///     host: String,
    /// }
    ///
    /// #[derive(Debug, Config)]
    /// struct AppConfig {
    ///     #[config]
    ///     database: DatabaseConfig,
    /// }
    ///
    /// // DB_HOST is not set
    /// # unsafe { std::env::remove_var("DB_HOST"); }
    /// let err = AppConfig::load().unwrap_err();
    /// // Will contain a Nested error wrapping the DatabaseConfig error
    /// ```
    Nested {
        /// The index of the field in the struct.
        field_idx: usize,
        /// The name of the nested configuration field.
        field_name: Option<String>,
        /// The error from loading the nested configuration.
        error: ConfigError,
    },
}
