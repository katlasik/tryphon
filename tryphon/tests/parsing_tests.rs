use std::env;
use tryphon::{Config, ConfigFieldError};

mod common;
use common::TEST_MUTEX;

fn clear_test_env_vars() {
    clear_test_env_vars!("DEBUG_MODE", "OPTIONAL_NUMBER");
}

#[derive(Debug, Config)]
struct AppConfig {
    #[env("DEBUG_MODE")]
    debug: bool,
    #[env("OPTIONAL_NUMBER")]
    optional_number: Option<i32>,
}

#[test]
fn test_boolean_parsing() {
    let _unused = TEST_MUTEX.lock().unwrap();

    clear_test_env_vars();

    unsafe {
        env::set_var("DEBUG_MODE", "false");
    }

    let config = AppConfig::load().expect("Failed to load config");
    assert_eq!(config.debug, false);
    assert_eq!(config.optional_number, None);

    unsafe {
        env::set_var("DEBUG_MODE", "true");
    }
    let config = AppConfig::load().expect("Failed to load config");
    assert_eq!(config.debug, true);
    assert_eq!(config.optional_number, None);

    unsafe {
        env::set_var("DEBUG_MODE", "xyz");
    }
    let config = AppConfig::load().expect_err("Expected to fail config");

    assert!(matches!(
      config.field_errors.first().expect("Expected 1 error"),
      ConfigFieldError::ParsingError {
        raw,
        ..
      } if raw == "xyz"
    ));

    clear_test_env_vars();
}

#[test]
fn test_type_parsing_errors() {
    let _unused = TEST_MUTEX.lock().unwrap();

    clear_test_env_vars();

    unsafe {
        env::set_var("DEBUG_MODE", "false");
        env::set_var("OPTIONAL_NUMBER", "not_a_number");
    }

    let config = AppConfig::load().expect_err("Expected to fail config");

    assert!(matches!(
      config.field_errors.first().expect("Expected 1 error"),
      ConfigFieldError::ParsingError {
        raw,
        ..
      } if raw == "not_a_number"
    ));

    clear_test_env_vars();
}
