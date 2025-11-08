/// A trait for decoding raw string values from environment variables into typed values.
///
/// This trait defines how to convert a raw string value from an environment variable
/// into a specific Rust type. Implement this trait to support custom types in your
/// configuration structs.
///
/// # Examples
///
/// ## Simple Custom Type
///
/// ```rust
/// use tryphon::{ConfigValueDecoder, Config};
///
/// #[derive(Debug, PartialEq)]
/// enum Environment {
///     Development,
///     Staging,
///     Production,
/// }
///
/// impl ConfigValueDecoder for Environment {
///     fn decode(raw: String) -> Result<Self, String> {
///         match raw.to_lowercase().as_str() {
///             "dev" | "development" => Ok(Environment::Development),
///             "staging" => Ok(Environment::Staging),
///             "prod" | "production" => Ok(Environment::Production),
///             _ => Err(format!("Unknown environment: {}", raw))
///         }
///     }
/// }
///
/// #[derive(Config)]
/// struct AppConfig {
///     #[env("ENV")]
///     environment: Environment,
/// }
///
/// # unsafe { std::env::set_var("ENV", "production"); }
/// # let config = AppConfig::load().unwrap();
/// # assert_eq!(config.environment, Environment::Production);
/// ```
///
pub trait ConfigValueDecoder {
    /// Decodes a raw string value into the target type.
    ///
    /// # Arguments
    ///
    /// * `raw` - The raw string value from the environment variable
    ///
    /// # Errors
    ///
    /// Returns String if the value cannot be parsed into
    /// the target type. Should include a helpful error message describing why parsing
    /// failed.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tryphon::ConfigValueDecoder;
    ///
    /// let result = u16::decode("8080".to_string());
    /// assert_eq!(result.unwrap(), 8080);
    /// ```
    fn decode(raw: String) -> Result<Self, String>
    where
        Self: Sized;
}
