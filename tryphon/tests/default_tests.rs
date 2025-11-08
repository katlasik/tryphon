use std::env;
use tryphon::{Config, Secret};

mod common;
use common::TEST_MUTEX;

#[derive(Debug, Config)]
struct DbConfig {
    #[env("DB_HOST")]
    #[default("localhost")]
    host: String,

    #[env("DB_PASSWORD")]
    password: Secret<String>,

    #[env("DB_PORT")]
    #[default(5432)]
    port: u16,

    #[env("DB_NAME")]
    database: String,
}

#[derive(Debug, Config)]
struct AppConfig {
    #[env("APP_NAME")]
    #[default("test-app")]
    name: String,

    #[env("APP_PORT")]
    #[env("PORT")]
    #[default(8080)]
    port: u16,

    #[env("DEBUG_MODE")]
    #[default(false)]
    debug: bool,

    #[env("LOG_LEVEL")]
    log_level: Option<String>,

    #[config]
    database: DbConfig,
}

fn clear_test_env_vars() {
    clear_test_env_vars!(
        "DB_HOST",
        "DB_PASSWORD",
        "DB_PORT",
        "DB_NAME",
        "APP_NAME",
        "APP_PORT",
        "PORT",
        "DEBUG_MODE",
        "LOG_LEVEL"
    );
}

#[test]
fn test_successful_loading_with_all_values() {
    let _unused = TEST_MUTEX.lock().unwrap();

    clear_test_env_vars();

    unsafe {
        env::set_var("DB_PASSWORD", "secret123");
        env::set_var("DB_NAME", "test_db");
        env::set_var("APP_PORT", "3000");
        env::set_var("DEBUG_MODE", "true");
        env::set_var("LOG_LEVEL", "info");
    }

    let config = AppConfig::load().expect("Failed to load config");

    assert_eq!(config.name, "test-app"); // default value
    assert_eq!(config.port, 3000);
    assert_eq!(config.debug, true);
    assert_eq!(config.log_level, Some("info".to_string()));
    assert_eq!(config.database.host, "localhost"); // default value
    assert_eq!(*config.database.password, "secret123");
    assert_eq!(config.database.port, 5432); // default value
    assert_eq!(config.database.database, "test_db");

    clear_test_env_vars();
}

#[test]
fn test_loading_with_custom_values_overriding_defaults() {
    let _unused = TEST_MUTEX.lock().unwrap();

    clear_test_env_vars();

    unsafe {
        env::set_var("APP_NAME", "custom-app");
        env::set_var("DB_HOST", "db.example.com");
        env::set_var("DB_PASSWORD", "pass456");
        env::set_var("DB_PORT", "3306");
        env::set_var("DB_NAME", "production_db");
    }

    let config = AppConfig::load().expect("Failed to load config");

    assert_eq!(config.name, "custom-app");
    assert_eq!(config.database.host, "db.example.com");
    assert_eq!(*config.database.password, "pass456");
    assert_eq!(config.database.port, 3306);
    assert_eq!(config.database.database, "production_db");

    clear_test_env_vars();
}

#[test]
fn test_default_values_used_when_no_env_set() {
    let _unused = TEST_MUTEX.lock().unwrap();

    clear_test_env_vars();

    unsafe {
        env::set_var("DB_PASSWORD", "pass");
        env::set_var("DB_NAME", "db");
    }
    // APP_NAME, APP_PORT, DEBUG_MODE, DB_HOST, DB_PORT all have defaults

    let config = AppConfig::load().expect("Failed to load config");

    assert_eq!(config.name, "test-app");
    assert_eq!(config.port, 8080);
    assert_eq!(config.debug, false);
    assert_eq!(config.database.host, "localhost");
    assert_eq!(config.database.port, 5432);

    clear_test_env_vars();
}
