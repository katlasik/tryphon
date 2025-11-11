use tryphon::ConfigValueDecoder;

#[derive(ConfigValueDecoder)]
struct Port(u16);

#[derive(ConfigValueDecoder)]
struct UserId(String);

fn main() {}
