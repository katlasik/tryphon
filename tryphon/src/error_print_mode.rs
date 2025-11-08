/// Controls the output format for [`ConfigError::pretty_print`].
///
/// Different modes are suitable for different contexts - List mode is more compact
/// and suitable for logs, while Table mode provides better visual structure for
/// terminal output and debugging.
///
/// # Examples
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
///         // Use List mode for compact output
///         eprintln!("{}", e.pretty_print(ErrorPrintMode::List));
///
///         // Or use Table mode for structured output
///         eprintln!("{}", e.pretty_print(ErrorPrintMode::Table));
///     }
/// }
/// ```
pub enum ErrorPrintMode {
    /// List mode - compact, plain-text format suitable for logs.
    ///
    /// Outputs errors as a simple newline-separated list with full error messages.
    /// Each line contains the complete error description with field path and details.
    ///
    /// Example output:
    /// ```text
    /// Found 2 configuration error(s):
    /// Missing value for field 'database.host', tried env vars: DATABASE_HOST
    /// Parsing error for env var 'PORT' for field 'database.port': invalid digit found in string (raw value: abc)
    /// ```
    List,

    /// Table mode - structured ASCII table format for terminal output.
    ///
    /// Outputs errors as an ASCII table with three columns: Field Name, Environment Variables,
    /// and Error Details. Provides better visual structure for multiple errors.
    ///
    /// Example output:
    /// ```text
    /// ┌──────────────┬────────────────────────┬─────────────────────────────┐
    /// │ Field Name   │ Environment Variables  │ Error Details               │
    /// ├──────────────┼────────────────────────┼─────────────────────────────┤
    /// │ database.host│ DATABASE_HOST          │ Required variable not set   │
    /// │ database.port│ PORT                   │ invalid digit found... (raw)│
    /// └──────────────┴────────────────────────┴─────────────────────────────┘
    /// ```
    Table,
}
