use tryphon::{Config, ConfigFieldError, ConfigValueDecoder, env_vars};


#[derive(Debug)]
struct Point {
    x: i32,
    y: i32
}

impl ConfigValueDecoder for Point {
    fn decode(raw: String) -> Result<Self, String> {
        let parts = raw.split('/').collect::<Vec<_>>();
        if parts.len() != 2 {
            return Err(format!("Invalid format for Point: {}", raw));
        }
        let x = parts[0].trim().parse::<i32>().map_err(|e| e.to_string())?;
        let y = parts[1].trim().parse::<i32>().map_err(|e| e.to_string())?;
        Ok(Point { x, y })
    }
}

#[derive(Config, Debug)]
struct Rectangle {
    #[env("TOP_LEFT")]
    #[default(Point { x: 0, y: 0 })]
    top_left: Point,
    #[env("BOTTOM_RIGHT")]
    bottom_right: Point,
}

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
    switch: SwitchState,
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

#[test]
#[env_vars(BOTTOM_RIGHT = "30/40")]
fn test_custom_decoder() {
  let config = Rectangle::load().expect("Failed to load config with custom decoders");

  assert_eq!(config.top_left.x, 0);
  assert_eq!(config.top_left.y, 0);
  assert_eq!(config.bottom_right.x, 30);
  assert_eq!(config.bottom_right.y, 40);
}

#[test]
#[env_vars(BOTTOM_RIGHT = "30x40")]
fn test_custom_decoder_fail() {
  let error = Rectangle::load().expect_err("Should have failed to load config with bad values");

  matches!(&error.field_errors[..], [ConfigFieldError::ParsingError { message, .. }] if message.contains("Invalid format for Point"));


}
