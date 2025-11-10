mod common;

use common::TEST_MUTEX;
use std::env;

use tryphon::{Config, ConfigValueDecoder};

#[derive(Debug, PartialEq, ConfigValueDecoder)]
enum LogLevel {
    Error,
    Warning,
    Info,
    Debug,
}

#[derive(Debug, Config)]
struct AppConfig {
    #[env("LOG_LEVEL")]
    log_level: LogLevel,
}

fn clear_test_env_vars() {
    unsafe {
        clear_test_env_vars!("LOG_LEVEL");
    }
}

#[test]
fn test_enum() {
    let _unused = TEST_MUTEX.lock().unwrap();

    clear_test_env_vars();

    unsafe {
        env::set_var("LOG_LEVEL", "debug");
    }

    let result = AppConfig::load().expect("failed to load config from env");

    assert_eq!(result.log_level, LogLevel::Debug);
}
