use std::env;
use tryphon::Config;

mod common;
use common::TEST_MUTEX;

#[derive(Debug, Config)]
struct FallbackConfig {
    #[env("PRIMARY_VALUE")]
    #[env("SECONDARY_VALUE")]
    #[env("TERTIARY_VALUE")]
    value: String,
}

fn clear_test_env_vars() {
    clear_test_env_vars!("PRIMARY_VALUE", "SECONDARY_VALUE", "TERTIARY_VALUE");
}

#[test]
fn test_fallback_environment_variables_primary() {
    let _unused = TEST_MUTEX.lock().unwrap();

    clear_test_env_vars();

    unsafe {
        env::set_var("PRIMARY_VALUE", "from_primary");
    }

    let config = FallbackConfig::load().expect("Failed to load config");
    assert_eq!(config.value, "from_primary");

    clear_test_env_vars();
}

#[test]
fn test_fallback_environment_variables_secondary() {
    let _unused = TEST_MUTEX.lock().unwrap();

    clear_test_env_vars();

    // Primary not set, secondary is set
    unsafe {
        env::set_var("SECONDARY_VALUE", "from_secondary");
    }

    let config = FallbackConfig::load().expect("Failed to load config");
    assert_eq!(config.value, "from_secondary");

    clear_test_env_vars();
}

#[test]
fn test_fallback_environment_variables_tertiary() {
    let _unused = TEST_MUTEX.lock().unwrap();

    clear_test_env_vars();

    // Only tertiary is set
    unsafe {
        env::set_var("TERTIARY_VALUE", "from_tertiary");
    }

    let config = FallbackConfig::load().expect("Failed to load config");
    assert_eq!(config.value, "from_tertiary");

    clear_test_env_vars();
}

#[test]
fn test_fallback_priority_order() {
    let _unused = TEST_MUTEX.lock().unwrap();

    clear_test_env_vars();

    // Set all three, should use primary
    unsafe {
        env::set_var("PRIMARY_VALUE", "primary");
        env::set_var("SECONDARY_VALUE", "secondary");
        env::set_var("TERTIARY_VALUE", "tertiary");
    }

    let config = FallbackConfig::load().expect("Failed to load config");
    assert_eq!(config.value, "primary");

    clear_test_env_vars();

    // Set only secondary and tertiary, should use secondary
    unsafe {
        env::set_var("SECONDARY_VALUE", "secondary");
        env::set_var("TERTIARY_VALUE", "tertiary");
    }

    let config = FallbackConfig::load().expect("Failed to load config");
    assert_eq!(config.value, "secondary");

    clear_test_env_vars();
}
