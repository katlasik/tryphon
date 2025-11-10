use tryphon::{Config, ConfigValueDecoder, ErrorPrintMode, Secret};

#[derive(Debug, Config)]
struct DbCredentials {
    #[env("DB_USER")]
    username: String,

    #[env("DB_PASSWORD")]
    password: Secret<String>,
}

#[derive(Debug, Config)]
struct DbConfig {
    #[config]
    credentials: DbCredentials,

    #[env("DB_HOST")]
    #[default("localhost")]
    host: String,

    #[env("DB_PORT")]
    #[default(5432)]
    port: u16,

    #[env("DB_NAME")]
    database: String,
}

#[derive(Debug, ConfigValueDecoder)]
enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

#[derive(Debug, Config)]
struct AppConfig {
    #[env("APP_HOST")]
    #[default("localhost")]
    host: String,

    #[env("APP_PORT")]
    #[env("PORT")]
    #[default(8080)]
    port: u16,

    #[env("ADMIN_EMAIL")]
    #[env("ADMINISTRATOR_EMAIL")]
    admin_email: String,

    #[env("LOG_LEVEL")]
    log_level: Option<LogLevel>,

    #[config]
    database: DbConfig,
}

fn example_app() -> Result<(), String> {
    match AppConfig::load() {
        Ok(config) => {
            println!("Starting server on {}:{}", config.host, config.port);
            println!(
                "Connected to the database as {}@{}:{}/{}",
                config.database.credentials.username,
                config.database.host,
                config.database.port,
                config.database.database
            );
            println!(
                "DB password hash: {}",
                config.database.credentials.password.hashed()
            );
            println!("Using admin email: {}.", config.admin_email);
            config.log_level.iter().for_each(|level| {
                println!("Log level set to: {:?}", level);
            });
            Ok(())
        }
        Err(e) => {
            eprintln!("{}", e.pretty_print(ErrorPrintMode::List));
            Err(e.to_string())
        }
    }
}

// Set up required environment variables to run the example
fn main() {
    println!("Running example_app...");
    example_app().unwrap();
}

#[cfg(test)]
mod tests {
  use tryphon::env_vars;

  #[test]
  #[env_vars(
    DB_USER="test_user",
    DB_PASSWORD="test_pass",
    DB_NAME="test_db",
    ADMIN_EMAIL="acme@acme.com",
  )]
  fn test() {
    assert!(super::example_app().is_ok());
  }


}
