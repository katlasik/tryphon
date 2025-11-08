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
        "LOG_LEVEL",
        "REQUIRED_FIELD",
    )
}

#[test]
fn test_nested_config_loading() {
    let _unused = TEST_MUTEX.lock().unwrap();

    clear_test_env_vars();

    unsafe {
        env::set_var("DB_PASSWORD", "nested_pass");
        env::set_var("DB_NAME", "nested_db");
    }

    let config = AppConfig::load().expect("Failed to load nested config");

    // Verify nested config was loaded
    assert_eq!(config.database.host, "localhost");
    assert_eq!(*config.database.password, "nested_pass");
    assert_eq!(config.database.port, 5432);
    assert_eq!(config.database.database, "nested_db");

    clear_test_env_vars();
}

#[test]
fn test_nested_config_missing_required_value() {
    let _unused = TEST_MUTEX.lock().unwrap();

    clear_test_env_vars();

    // DB_PASSWORD is required in nested config but not set
    unsafe {
        env::set_var("DB_NAME", "nested_db");
    }

    let result = AppConfig::load();

    assert!(
        result.is_err(),
        "Expected error when nested config has missing required value"
    );

    clear_test_env_vars();
}
