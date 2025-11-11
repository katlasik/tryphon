use tryphon::ConfigValueDecoder;

#[derive(ConfigValueDecoder)]
enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

fn main() {}
