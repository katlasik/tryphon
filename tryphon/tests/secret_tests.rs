use std::env;
use tryphon::Config;
use tryphon::Secret;

mod common;
use common::TEST_MUTEX;

#[derive(Config, Debug)]
struct AppConfig {
    #[env("PASSWORD")]
    password: Secret<String>,
}

fn clear_test_env_vars() {
    clear_test_env_vars!("OPTIONAL_STRING", "OPTIONAL_NUMBER", "OPTIONAL_BOOL");
}

#[test]
fn test_secret_masking() {
    let _unused = TEST_MUTEX.lock().unwrap();

    clear_test_env_vars();

    unsafe {
        env::set_var("PASSWORD", "super_secret_password");
        env::set_var("DEBUG_MODE", "false");
    }

    let config = AppConfig::load().expect("Failed to load config");

    // Secret should not reveal the actual value in debug output
    let debug_output = format!("{:?}", config.password);
    assert!(
        !debug_output.contains("super_secret_password"),
        "Secret value should be masked in debug output"
    );
    assert!(
        debug_output.starts_with("Secret("),
        "Secret should show Secret(...) format"
    );

    assert_eq!(*config.password, "super_secret_password");

    clear_test_env_vars();
}
