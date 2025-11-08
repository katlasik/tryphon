use std::env;
use tryphon::Config;

mod common;
use common::TEST_MUTEX;

#[derive(Debug, Config)]
struct OptionalConfig {
    #[env("REQUIRED_FIELD")]
    required: String,

    #[env("OPTIONAL_STRING")]
    optional_string: Option<String>,

    #[env("OPTIONAL_NUMBER")]
    optional_number: Option<i32>,

    #[env("OPTIONAL_BOOL")]
    optional_bool: Option<bool>,
}

fn clear_test_env_vars() {
    clear_test_env_vars!("OPTIONAL_STRING", "OPTIONAL_NUMBER", "OPTIONAL_BOOL");
}

#[test]
fn test_optional_values_all_set() {
    let _unused = TEST_MUTEX.lock().unwrap();

    clear_test_env_vars();

    unsafe {
        env::set_var("REQUIRED_FIELD", "required_value");
        env::set_var("OPTIONAL_STRING", "optional_value");
        env::set_var("OPTIONAL_NUMBER", "42");
        env::set_var("OPTIONAL_BOOL", "true");
    }

    let config = OptionalConfig::load().expect("Failed to load config");

    assert_eq!(config.required, "required_value");
    assert_eq!(config.optional_string, Some("optional_value".to_string()));
    assert_eq!(config.optional_number, Some(42));
    assert_eq!(config.optional_bool, Some(true));

    clear_test_env_vars();
}

#[test]
fn test_optional_values_none_set() {
    let _unused = TEST_MUTEX.lock().unwrap();

    clear_test_env_vars();

    unsafe {
        env::set_var("REQUIRED_FIELD", "required_value");
    }
    // Optional fields not set

    let config = OptionalConfig::load().expect("Failed to load config");

    assert_eq!(config.required, "required_value");
    assert_eq!(config.optional_string, None);
    assert_eq!(config.optional_number, None);
    assert_eq!(config.optional_bool, None);

    clear_test_env_vars();
}

#[test]
fn test_optional_values_partially_set() {
    let _unused = TEST_MUTEX.lock().unwrap();

    clear_test_env_vars();

    unsafe {
        env::set_var("REQUIRED_FIELD", "required_value");
        env::set_var("OPTIONAL_NUMBER", "99");
    }
    // Other optional fields not set

    let config = OptionalConfig::load().expect("Failed to load config");

    assert_eq!(config.required, "required_value");
    assert_eq!(config.optional_string, None);
    assert_eq!(config.optional_number, Some(99));
    assert_eq!(config.optional_bool, None);

    clear_test_env_vars();
}
