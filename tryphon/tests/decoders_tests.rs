use tryphon::{Config, ConfigFieldError, ConfigValueDecoder, env_vars};

#[derive(ConfigValueDecoder, Debug)]
struct Flag(bool);

#[derive(ConfigValueDecoder, Debug)]
struct SystemPort(u16);

#[derive(ConfigValueDecoder, Debug, PartialEq)]
enum SwitchState {
    On,
    Off,
}

#[derive(Debug, Config)]
struct AppConfig {
    #[env("FLAG")] #[default(Flag(false))]
    flag: Flag,
    #[env("SYSTEM_PORT")] #[default(SystemPort(80))]
    port: SystemPort,
    #[env("SWITCH_STATE")] #[default(SwitchState::Off)]
    switch: SwitchState
}

#[test]
#[env_vars(FLAG = "true", SYSTEM_PORT = "8080", SWITCH_STATE = "on")]
fn test_derived_decoders() {
    let config = AppConfig::load().expect("Failed to load config with custom decoders");

    assert!(config.flag.0);
    assert_eq!(config.port.0, 8080);
    assert_eq!(config.switch, SwitchState::On);
}

#[test]
#[env_vars(FLAG = "not_bool", SYSTEM_PORT = "not_a_number", SWITCH_STATE = "bad")]
fn test_derived_decoders_failure() {
    let error = AppConfig::load().expect_err("Should have failed to load config with bad values");

    assert_eq!(error.field_errors.len(), 3);

    if let [
        ConfigFieldError::ParsingError { raw: raw_flag, .. },
        ConfigFieldError::ParsingError { raw: raw_port, .. },
        ConfigFieldError::ParsingError {
            raw: raw_switch, ..
        },
    ] = &error.field_errors[..]
    {
        assert_eq!(raw_flag, "not_bool");
        assert_eq!(raw_port, "not_a_number");
        assert_eq!(raw_switch, "bad");
    } else {
        panic!("Expected parsing errors for all fields");
    }
}

#[test]
fn test_derived_decoders_defaults() {
  let config = AppConfig::load().expect("Failed to load config with custom decoders");

  assert!(!config.flag.0);
  assert_eq!(config.port.0, 80);
  assert_eq!(config.switch, SwitchState::Off);
}
