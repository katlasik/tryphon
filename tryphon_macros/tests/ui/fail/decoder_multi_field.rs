use tryphon::ConfigValueDecoder;

#[derive(ConfigValueDecoder)]
struct BadNewtype {
    x: String,
    y: i32,
}

fn main() {}
