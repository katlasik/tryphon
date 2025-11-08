use crate::config_field_error::ConfigFieldError;
use crate::error_print_mode::ErrorPrintMode;
use crate::printer::list_printer::ListPrinter;
use crate::printer::table_printer::TablePrinter;
use std::fmt::{Display, Formatter};

/// Error returned when configuration loading fails.
///
/// Contains a collection of all field-level errors encountered during the loading process.
/// This allows you to see and handle all configuration problems at once rather than
/// stopping at the first error.
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
///     port: u16,
/// }
///
/// # unsafe { std::env::set_var("DATABASE_URL", "postgres://localhost"); }
/// # unsafe { std::env::set_var("PORT", "8080"); }
/// match AppConfig::load() {
///     Ok(config) => {
///         // Use the configuration
/// #       assert_eq!(config.database_url, "postgres://localhost");
///     }
///     Err(config_error) => {
///         eprintln!("Found {} configuration error(s):", config_error.field_errors.len());
///         for error in &config_error.field_errors {
///             eprintln!("  - {:?}", error);
///         }
///     }
/// }
/// ```
#[derive(Debug, Clone)]
pub struct ConfigError {
    /// A vector of all field-level errors encountered during configuration loading.
    ///
    /// Each error represents a problem with a specific field, such as a missing
    /// environment variable or a parsing failure.
    pub field_errors: Vec<ConfigFieldError>,
}

impl ConfigError {
    /// Formats configuration errors in a human-readable format.
    ///
    /// This method provides two formatting modes via [`ErrorPrintMode`]:
    /// - [`ErrorPrintMode::List`] - Compact bulleted list format, ideal for log files
    /// - [`ErrorPrintMode::Table`] - ASCII table format with columns, ideal for terminal output
    ///
    /// Both formats include all error details including nested errors from nested configuration structs.
    ///
    /// # Arguments
    ///
    /// * `mode` - The output format mode to use
    ///
    /// # Returns
    ///
    /// A formatted string containing all configuration errors with a header "Configuration errors:"
    ///
    /// # Examples
    ///
    /// ## List Mode
    ///
    /// ```rust
    /// use tryphon::{Config, ErrorPrintMode};
    ///
    /// #[derive(Config)]
    /// struct AppConfig {
    ///     #[env("MISSING_VAR")]
    ///     value: String,
    /// }
    ///
    /// match AppConfig::load() {
    ///     Ok(_) => println!("Config loaded"),
    ///     Err(e) => {
    ///         // Prints: Found 1 configuration error(s):
    ///         // Missing value for field 'value', tried env vars: MISSING_VAR
    ///         eprintln!("{}", e.pretty_print(ErrorPrintMode::List));
    ///     }
    /// }
    /// ```
    ///
    /// ## Table Mode
    ///
    /// ```rust
    /// use tryphon::{Config, ErrorPrintMode};
    ///
    /// #[derive(Config)]
    /// struct AppConfig {
    ///     #[env("PORT")]
    ///     port: u16,
    /// }
    ///
    /// # unsafe { std::env::set_var("PORT", "invalid"); }
    /// match AppConfig::load() {
    ///     Ok(_) => {},
    ///     Err(e) => {
    ///         // Prints a formatted ASCII table:
    ///         // ┌────────────┬────────────────────────┬─────────────────────────┐
    ///         // │ Field Name │ Environment Variables  │ Error Details           │
    ///         // ├────────────┼────────────────────────┼─────────────────────────┤
    ///         // │ port       │ PORT                   │ invalid digit found...  │
    ///         // └────────────┴────────────────────────┴─────────────────────────┘
    ///         eprintln!("{}", e.pretty_print(ErrorPrintMode::Table));
    ///     }
    /// }
    /// # unsafe { std::env::remove_var("PORT"); }
    /// ```
    ///
    /// [`ErrorPrintMode`]: crate::ErrorPrintMode
    /// [`ErrorPrintMode::List`]: crate::ErrorPrintMode::List
    /// [`ErrorPrintMode::Table`]: crate::ErrorPrintMode::Table
    pub fn pretty_print(&self, mode: ErrorPrintMode) -> String {
        match mode {
            ErrorPrintMode::List => ListPrinter::new().print(&self.field_errors),
            ErrorPrintMode::Table => TablePrinter::new().print(&self.field_errors),
        }
    }
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.pretty_print(ErrorPrintMode::List))
    }
}
