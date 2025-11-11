use tryphon::ConfigValueDecoder;

#[derive(ConfigValueDecoder)]
enum BadLevel {
    Complex { x: i32, y: i32 },
}

fn main() {}
